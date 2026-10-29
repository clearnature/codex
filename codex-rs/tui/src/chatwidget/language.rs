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
use std::path::Path;

const LANGUAGE_PICKER_VIEW_ID: &str = "language-picker";
const LANGUAGE_SCOPE_VIEW_ID: &str = "language-scope-picker";

struct LanguageEntry {
    lang: Lang,
    name: &'static str,
    search: &'static str,
}

const LANGUAGE_ENTRIES: [LanguageEntry; 3] = [
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
    LanguageEntry {
        lang: Lang::ZhHant,
        name: "繁體中文 (Traditional Chinese) [zh-TW]",
        search: "traditional 繁體中文 繁体中文 zh-tw zh-hant zh-hk tw",
    },
];

fn entry_for(lang: Lang) -> &'static LanguageEntry {
    match lang {
        Lang::En => &LANGUAGE_ENTRIES[0],
        Lang::Zh => &LANGUAGE_ENTRIES[1],
        Lang::ZhHant => &LANGUAGE_ENTRIES[2],
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
        "zh-tw" | "zh-hant" | "zh-hk" | "tw" | "繁體" | "繁体" | "繁體中文" | "繁体中文" => {
            Some(Lang::ZhHant)
        }
        _ => None,
    }
}

/// Parsed `/language` argument line: the target language (if any), the
/// persistence flag (`--config` / `--session`), and whether the line was
/// invalid (conflicting flags or an unrecognized language).
pub(crate) struct LanguageRequest {
    pub lang: Option<Lang>,
    pub persist: Option<bool>,
    pub conflicting_flags: bool,
}

pub(crate) fn parse_language_request(args: &str) -> LanguageRequest {
    let mut persist = None;
    let mut conflicting_flags = false;
    let mut remaining = String::new();
    for token in args.split_whitespace() {
        match token {
            "--config" => {
                if persist == Some(false) {
                    conflicting_flags = true;
                }
                persist = Some(true);
            }
            "--session" => {
                if persist == Some(true) {
                    conflicting_flags = true;
                }
                persist = Some(false);
            }
            other => {
                if !remaining.is_empty() {
                    remaining.push(' ');
                }
                remaining.push_str(other);
            }
        }
    }
    let lang = if remaining.is_empty() {
        None
    } else {
        parse_language_arg(&remaining)
    };
    LanguageRequest {
        lang,
        persist,
        conflicting_flags,
    }
}

/// The comma-joined language list shown when an argument is not recognized.
pub(crate) fn available_language_list() -> String {
    LANGUAGE_ENTRIES
        .iter()
        .map(|entry| entry.name.to_string())
        .collect::<Vec<_>>()
        .join("、")
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

/// The marker pair that bounds the managed output-language section inside
/// the global AGENTS.md. Everything between them is owned by `/language
/// output` and replaced wholesale; the rest of the file is untouched.
const OUTPUT_LANGUAGE_MARKER_BEGIN: &str = "<!-- codex:output-language:begin -->";
const OUTPUT_LANGUAGE_MARKER_END: &str = "<!-- codex:output-language:end -->";

/// Sanitizes a language string for safe embedding in an HTML-comment marker.
fn sanitize_output_language(language: &str) -> String {
    language
        .replace(['\r', '\n'], " ")
        .replace("--!>", "")
        .replace("--", "")
}

fn output_language_rule(language: &str) -> String {
    let safe = sanitize_output_language(language);
    format!(
        "{OUTPUT_LANGUAGE_MARKER_BEGIN}\n<!-- output-language: {safe} -->\n\n## Output language\n\nAlways respond in **{safe}** regardless of the user's input language, unless the user explicitly asks for a different language for the rest of the conversation. Do not translate or rewrite code blocks, CLI commands, file paths, stack traces, logs, JSON keys, identifiers, or exact quoted text.\n{OUTPUT_LANGUAGE_MARKER_END}"
    )
}

/// Replaces or appends the managed output-language section in the global
/// AGENTS.md. `None` removes the section entirely.
/// Test shim: the section management is otherwise exercised only through the
/// command handler.
#[cfg(test)]
pub(crate) fn __test_write_output_language_section(codex_home: &Path, language: Option<&str>) {
    let _ = write_output_language_section(codex_home, language);
}

fn write_output_language_section(codex_home: &Path, language: Option<&str>) -> std::io::Result<()> {
    let agents_md = codex_home.join("AGENTS.md");
    let existing = std::fs::read_to_string(&agents_md).unwrap_or_default();
    // Cut the managed section out (begin-marker to end-marker, or to EOF when
    // unterminated), keeping everything before and after it intact.
    let stripped = match existing.find(OUTPUT_LANGUAGE_MARKER_BEGIN) {
        Some(start) => {
            let tail_start = match existing[start..].find(OUTPUT_LANGUAGE_MARKER_END) {
                Some(end_rel) => start + end_rel + OUTPUT_LANGUAGE_MARKER_END.len(),
                None => existing.len(),
            };
            let head = existing[..start].trim_end();
            let tail = existing[tail_start..].trim_start_matches('\n');
            match (head.is_empty(), tail.is_empty()) {
                (true, _) => tail.to_string(),
                (_, true) => head.to_string(),
                (false, false) => format!("{head}\n\n{tail}"),
            }
        }
        None => existing.trim_end().to_string(),
    };
    let merged = match language {
        Some(lang) => {
            let section = output_language_rule(lang);
            if stripped.is_empty() {
                format!("{section}\n")
            } else {
                format!("{stripped}\n\n{section}\n")
            }
        }
        None if stripped.is_empty() => String::new(),
        None => format!("{stripped}\n"),
    };
    std::fs::write(&agents_md, merged)
}

/// Reads the managed output language back out (for `/language output` status).
fn read_output_language_section(codex_home: &Path) -> Option<String> {
    let existing = std::fs::read_to_string(codex_home.join("AGENTS.md")).ok()?;
    let start = existing.find(OUTPUT_LANGUAGE_MARKER_BEGIN)?;
    let rest = &existing[start..];
    let marker = "<!-- output-language: ";
    let value_start = rest.find(marker)? + marker.len();
    let value_end = rest[value_start..].find(" -->")? + value_start;
    Some(rest[value_start..value_end].trim().to_string())
}

impl ChatWidget {
    /// `/language output` -- show, set, or clear the LLM output language.
    /// The rule lives in a managed section of the global AGENTS.md, so it is
    /// carried to model context by the existing user-instructions channel and
    /// applies to new sessions.
    pub(crate) fn handle_output_language_command(&mut self, rest: &str) {
        let codex_home = self.local_settings.codex_home.clone();
        match rest {
            "" => match read_output_language_section(&codex_home) {
                Some(lang) => {
                    self.add_info_message(
                        tr(current(), "LLM output language: {0}").replace("{0}", &lang),
                        None,
                    );
                }
                None => {
                    self.add_info_message(
                        tr(
                            current(),
                            "LLM output language is not set; the model follows your input language.",
                        )
                        .to_string(),
                        None,
                    );
                }
            },
            "none" | "clear" | "清除" => match write_output_language_section(&codex_home, None) {
                Ok(()) => self.add_info_message(
                    tr(current(), "LLM output language cleared.").to_string(),
                    None,
                ),
                Err(err) => self.add_warning_message(tr_with(
                    current(),
                    "Failed to update AGENTS.md: {0}",
                    &[&err.to_string()],
                )),
            },
            language => match write_output_language_section(&codex_home, Some(language)) {
                Ok(()) => {
                    let message = tr(
                        current(),
                        "LLM output language set to {0}; applies to new sessions.",
                    )
                    .replace("{0}", language);
                    self.add_info_message(message, None);
                }
                Err(err) => self.add_warning_message(tr_with(
                    current(),
                    "Failed to update AGENTS.md: {0}",
                    &[&err.to_string()],
                )),
            },
        }
    }

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
            Lang::ZhHant => "zh-TW",
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

    /// `/language <lang> --config|--session` -- switch now and take the
    /// persistence decision from the flag, skipping the scope prompt.
    pub(crate) fn apply_interface_language_direct(&mut self, lang: Lang, persist: bool) {
        set_current(lang);
        self.app_event_tx
            .send(AppEvent::LanguageScopeSelected { lang, persist });
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
