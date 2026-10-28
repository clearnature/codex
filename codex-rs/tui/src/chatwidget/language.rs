//! Chat widget helpers for the `/language` interface-language picker.
//!
//! Ported from qwen-code's `/language` command: an interactive, multi-level
//! flow -- pick a language, then choose whether to keep it only for this
//! session or persist it into `config.toml` (the second link of the locale
//! chain implemented by `codex-i18n`).

use super::*;
use crate::legacy_core::config::edit::ConfigEditsBuilder;
use codex_i18n::Lang;
use codex_i18n::current;
use codex_i18n::set_current;
use codex_i18n::tr;
use codex_i18n::tr_with;

const LANGUAGE_PICKER_VIEW_ID: &str = "language-picker";
const LANGUAGE_SCOPE_VIEW_ID: &str = "language-scope-picker";

struct LanguageEntry {
    lang: Lang,
    name: &'static str,
    search: &'static str,
}

const LANGUAGE_ENTRIES: [LanguageEntry; 2] = [
    LanguageEntry {
        lang: Lang::En,
        name: "English [en]",
        search: "english en en-us",
    },
    LanguageEntry {
        lang: Lang::Zh,
        name: "中文 (Chinese) [zh]",
        search: "chinese 中文 简体中文 zh zh-cn zh-hans cn",
    },
];

fn entry_for(lang: Lang) -> &'static LanguageEntry {
    match lang {
        Lang::En => &LANGUAGE_ENTRIES[0],
        Lang::Zh => &LANGUAGE_ENTRIES[1],
    }
}

/// Map a `/language <arg>` argument to a language. Deliberately stricter than
/// [`codex_i18n::parse_lang`], which falls back to English for *any* unknown
/// input -- here an unrecognized argument must fall through to the picker
/// instead of silently switching to English.
pub(crate) fn parse_language_arg(raw: &str) -> Option<Lang> {
    let normalized = raw.trim().to_ascii_lowercase().replace('_', "-");
    match normalized.as_str() {
        "en" | "eng" | "english" | "en-us" => Some(Lang::En),
        "zh" | "cn" | "zh-cn" | "zh-hans" | "chinese" | "中文" | "简体" | "简体中文" => {
            Some(Lang::Zh)
        }
        _ => None,
    }
}

fn build_language_picker_params() -> crate::bottom_pane::SelectionViewParams {
    let current_lang = current();
    let mut initial_selected_idx = None;
    let items = LANGUAGE_ENTRIES
        .iter()
        .enumerate()
        .map(|(idx, entry)| {
            let is_current = entry.lang == current_lang;
            if is_current {
                initial_selected_idx = Some(idx);
            }
            let lang = entry.lang;
            let actions: Vec<crate::bottom_pane::SelectionAction> = vec![Box::new(move |tx| {
                tx.send(AppEvent::InterfaceLanguageSelected { lang });
            })];
            crate::bottom_pane::SelectionItem {
                name: entry.name.to_string(),
                description: Some(tr(current(), "switch the interface language").to_string()),
                is_current,
                dismiss_on_select: true,
                search_value: Some(entry.search.to_string()),
                actions,
                ..Default::default()
            }
        })
        .collect();

    crate::bottom_pane::SelectionViewParams {
        view_id: Some(LANGUAGE_PICKER_VIEW_ID),
        title: Some(tr(current(), "Select Language").to_string()),
        subtitle: Some(tr(current(), "Choose the interface language.").to_string()),
        footer_hint: Some(crate::bottom_pane::popup_consts::standard_popup_hint_line()),
        items,
        is_searchable: true,
        search_placeholder: Some(tr(current(), "Type to filter languages...").to_string()),
        initial_selected_idx,
        ..Default::default()
    }
}

fn build_language_scope_params(lang: Lang) -> crate::bottom_pane::SelectionViewParams {
    let name = entry_for(lang).name;
    let scope_items: [(bool, &str, &str); 2] = [
        (
            false,
            tr(current(), "Session only"),
            tr(current(), "Apply until you quit Codex."),
        ),
        (
            true,
            tr(current(), "Write to config.toml"),
            tr(current(), "Save the choice so it persists across restarts."),
        ),
    ];
    let items = scope_items
        .into_iter()
        .map(|(persist, title, description)| {
            let actions: Vec<crate::bottom_pane::SelectionAction> = vec![Box::new(move |tx| {
                tx.send(AppEvent::LanguageScopeSelected { lang, persist });
            })];
            crate::bottom_pane::SelectionItem {
                name: title.to_string(),
                description: Some(format!("{description} ({name})")),
                is_current: false,
                dismiss_on_select: true,
                search_value: None,
                actions,
                ..Default::default()
            }
        })
        .collect();

    crate::bottom_pane::SelectionViewParams {
        view_id: Some(LANGUAGE_SCOPE_VIEW_ID),
        title: Some(tr(current(), "Persistence").to_string()),
        subtitle: Some(tr(current(), "Keep the new interface language after restart?").to_string()),
        footer_hint: Some(crate::bottom_pane::popup_consts::standard_popup_hint_line()),
        items,
        is_searchable: false,
        search_placeholder: None,
        initial_selected_idx: Some(0),
        ..Default::default()
    }
}

impl ChatWidget {
    /// `/language` -- open the first-level language picker.
    pub(crate) fn open_language_picker(&mut self) {
        let params = build_language_picker_params();
        self.bottom_pane.show_selection_view(params);
    }

    /// `/language <arg>` -- apply a parsed language directly, skipping the
    /// first-level picker but still prompting for the persistence scope.
    pub(crate) fn apply_interface_language(&mut self, lang: Lang) {
        set_current(lang);
        let params = build_language_scope_params(lang);
        self.bottom_pane.show_selection_view(params);
    }

    /// Second-level choice: keep session-only, or write `locale` into
    /// config.toml so the five-level locale chain resolves to it on startup.
    pub(crate) fn persist_interface_language(&mut self, lang: Lang) {
        let code = match lang {
            Lang::Zh => "zh-CN",
            Lang::En => "en",
        };
        let codex_home = self.local_settings.codex_home.clone();
        let tx = self.app_event_tx.clone();
        tokio::spawn(async move {
            let result = ConfigEditsBuilder::new(codex_home.as_path())
                .set_locale(code)
                .apply()
                .await;
            let result = result.map_err(|error| {
                tr_with(
                    current(),
                    "Failed to write config.toml: {0}",
                    &[&error.to_string()],
                )
            });
            tx.send(AppEvent::LanguagePersisted { lang, result });
        });
    }

    pub(crate) fn show_language_status(&mut self, lang: Lang, persisted: bool) {
        let name = entry_for(lang).name;
        let text = if persisted {
            tr(
                current(),
                "Interface language changed and saved to config.toml.",
            )
        } else {
            tr(current(), "Interface language changed (session only).")
        };
        self.add_info_message(format!("{name}: {text}"), None);
    }
}
