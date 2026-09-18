//! Shortcut picker construction for `/keymap`.

use codex_config::types::TuiKeymap;
use codex_i18n::current;
use codex_i18n::tr;
use codex_i18n::tr_with;
use ratatui::style::Styled;
use ratatui::style::Stylize;
use ratatui::text::Line;
use ratatui::text::Span;
use std::sync::OnceLock;
use unicode_width::UnicodeWidthStr;

use crate::app_event::AppEvent;
use crate::bottom_pane::ColumnWidthMode;
use crate::bottom_pane::SelectionItem;
use crate::bottom_pane::SelectionRowDisplay;
use crate::bottom_pane::SelectionTab;
use crate::bottom_pane::SelectionViewParams;
use crate::keymap::RuntimeKeymap;
use crate::render::renderable::ColumnRenderable;
use crate::render::renderable::Renderable;
use crate::style::accent_style;

use super::actions::KeymapActionFilter;
use super::actions::action_label;
use super::actions::format_action_binding_summary;
use super::actions::keymap_actions;
use super::has_custom_binding;

pub(crate) const KEYMAP_PICKER_VIEW_ID: &str = "keymap-picker";
pub(super) const KEYMAP_ALL_TAB_ID: &str = "all-shortcuts";
pub(super) const KEYMAP_COMMON_TAB_ID: &str = "common-shortcuts";
pub(super) const KEYMAP_CUSTOM_TAB_ID: &str = "custom-shortcuts";
pub(super) const KEYMAP_UNBOUND_TAB_ID: &str = "unbound-shortcuts";
pub(super) const KEYMAP_DEBUG_TAB_ID: &str = "debug-shortcuts";
const KEYMAP_CONTEXT_LABEL_WIDTH: usize = 12;
const KEYMAP_ROW_PREFIX_WIDTH: usize = KEYMAP_CONTEXT_LABEL_WIDTH + 3;

#[derive(Clone, Debug)]
struct KeymapActionRow {
    context: &'static str,
    context_label: &'static str,
    action: &'static str,
    label: String,
    description: &'static str,
    binding_summary: String,
    custom_binding: bool,
}

impl KeymapActionRow {
    fn is_unbound(&self) -> bool {
        self.binding_summary == "unbound"
    }
}

pub(super) struct KeymapContextTab {
    id: &'static str,
    label: &'static str,
    description: &'static str,
    contexts: &'static [&'static str],
}

const KEYMAP_COMMON_ACTIONS: &[(&str, &str)] = &[
    ("composer", "submit"),
    ("chat", "interrupt_turn"),
    ("editor", "insert_newline"),
    ("composer", "queue"),
    ("global", "toggle_fast_mode"),
    ("global", "open_external_editor"),
    ("global", "copy"),
    ("global", "toggle_vim_mode"),
    ("editor", "delete_backward_word"),
    ("editor", "delete_forward_word"),
    ("editor", "move_word_left"),
    ("editor", "move_word_right"),
    ("global", "open_transcript"),
    ("pager", "close"),
    ("pager", "page_up"),
    ("pager", "page_down"),
    ("approval", "open_fullscreen"),
    ("approval", "approve"),
    ("approval", "approve_for_session"),
    ("approval", "decline"),
    ("approval", "cancel"),
];

/// The per-context tabs, rendered in the current UI language.
///
/// A cached function rather than a `const`, per §3.6 of
/// `docs/plan/i18n-design.md`: the labels and descriptions have to pass through
/// `tr`, which a `const` cannot call. The language is fixed per process, so the
/// table is built once and handed out as a `&'static [..]`.
pub(super) fn keymap_context_tabs() -> &'static [KeymapContextTab] {
    static TABS: OnceLock<Vec<KeymapContextTab>> = OnceLock::new();
    TABS.get_or_init(|| {
        vec![
            KeymapContextTab {
                id: "app-shortcuts",
                label: tr(current(), "App"),
                description: tr(current(), "Global and chat-level shortcuts."),
                contexts: &["global", "chat"],
            },
            KeymapContextTab {
                id: "composer-shortcuts",
                label: tr(current(), "Composer"),
                description: tr(current(), "Composer submission and queue shortcuts."),
                contexts: &["composer"],
            },
            KeymapContextTab {
                id: "editor-shortcuts",
                label: tr(current(), "Editor"),
                description: tr(current(), "Inline editor movement and editing shortcuts."),
                contexts: &["editor"],
            },
            KeymapContextTab {
                id: "vim-shortcuts",
                label: tr(current(), "Vim"),
                description: tr(current(), "Vim normal-mode and operator shortcuts."),
                contexts: &[
                    "vim_normal",
                    "vim_operator",
                    "vim_search",
                    "vim_text_object",
                ],
            },
            KeymapContextTab {
                id: "navigation-shortcuts",
                label: tr(current(), "Navigation"),
                description: tr(current(), "Pager and selection-list navigation shortcuts."),
                contexts: &["pager", "list"],
            },
            KeymapContextTab {
                id: "agents-shortcuts",
                label: tr(current(), "Agents"),
                description: tr(current(), "Shared agents dashboard shortcuts."),
                contexts: &["agents"],
            },
            KeymapContextTab {
                id: "approval-shortcuts",
                label: tr(current(), "Approval"),
                description: tr(current(), "Approval prompt shortcuts."),
                contexts: &["approval"],
            },
        ]
    })
}

#[cfg(test)]
pub(crate) fn build_keymap_picker_params(
    runtime_keymap: &RuntimeKeymap,
    keymap_config: &TuiKeymap,
) -> SelectionViewParams {
    build_keymap_picker_params_with_filter(
        runtime_keymap,
        keymap_config,
        KeymapActionFilter::default(),
    )
}

pub(crate) fn build_keymap_picker_params_with_filter(
    runtime_keymap: &RuntimeKeymap,
    keymap_config: &TuiKeymap,
    action_filter: KeymapActionFilter,
) -> SelectionViewParams {
    build_keymap_picker_params_for_action(
        runtime_keymap,
        keymap_config,
        action_filter,
        /*selected_action*/ None,
    )
}

#[cfg(test)]
pub(crate) fn build_keymap_picker_params_for_selected_action(
    runtime_keymap: &RuntimeKeymap,
    keymap_config: &TuiKeymap,
    context: &str,
    action: &str,
) -> SelectionViewParams {
    build_keymap_picker_params_for_selected_action_with_filter(
        runtime_keymap,
        keymap_config,
        KeymapActionFilter::default(),
        context,
        action,
    )
}

pub(crate) fn build_keymap_picker_params_for_selected_action_with_filter(
    runtime_keymap: &RuntimeKeymap,
    keymap_config: &TuiKeymap,
    action_filter: KeymapActionFilter,
    context: &str,
    action: &str,
) -> SelectionViewParams {
    build_keymap_picker_params_for_action(
        runtime_keymap,
        keymap_config,
        action_filter,
        Some((context, action)),
    )
}

fn build_keymap_picker_params_for_action(
    runtime_keymap: &RuntimeKeymap,
    keymap_config: &TuiKeymap,
    action_filter: KeymapActionFilter,
    selected_action: Option<(&str, &str)>,
) -> SelectionViewParams {
    let rows = build_keymap_rows(runtime_keymap, keymap_config, action_filter);
    let total = rows.len();
    let custom_count = rows.iter().filter(|row| row.custom_binding).count();
    let unbound_count = rows.iter().filter(|row| row.is_unbound()).count();
    let initial_selected_idx = selected_action.and_then(|(context, action)| {
        rows.iter()
            .position(|row| row.context == context && row.action == action)
    });
    let name_column_width = rows
        .iter()
        .map(|row| KEYMAP_ROW_PREFIX_WIDTH + UnicodeWidthStr::width(row.label.as_str()))
        .max();

    let mut tabs = Vec::new();
    tabs.push(SelectionTab {
        id: KEYMAP_ALL_TAB_ID.to_string(),
        label: tr(current(), "All").to_string(),
        header: keymap_header(
            tr(current(), "All configurable shortcuts.").to_string(),
            tr_with(
                current(),
                "{0} actions, {1} customized, {2} unbound.",
                &[
                    &total.to_string(),
                    &custom_count.to_string(),
                    &unbound_count.to_string(),
                ],
            ),
        ),
        items: keymap_selection_items(
            rows.iter(),
            tr(current(), "No shortcuts available"),
            tr(current(), "No configurable shortcuts are available."),
        ),
    });

    let common_rows = keymap_common_rows(&rows);
    let common_count = common_rows.len();
    tabs.push(SelectionTab {
        id: KEYMAP_COMMON_TAB_ID.to_string(),
        label: tr(current(), "Common").to_string(),
        header: keymap_header(
            tr(current(), "Frequently customized shortcuts.").to_string(),
            action_count_line(common_count),
        ),
        items: keymap_selection_items(
            common_rows,
            tr(current(), "No common shortcuts"),
            tr(current(), "No common shortcut actions are available."),
        ),
    });

    let custom_rows = rows
        .iter()
        .filter(|row| row.custom_binding)
        .collect::<Vec<_>>();
    tabs.push(SelectionTab {
        id: KEYMAP_CUSTOM_TAB_ID.to_string(),
        label: tr_with(current(), "Customized ({0})", &[&custom_count.to_string()]),
        header: keymap_header(
            tr(current(), "Root-level shortcut overrides.").to_string(),
            action_count_line(custom_count),
        ),
        items: keymap_selection_items(
            custom_rows,
            tr(current(), "No customized shortcuts"),
            tr(
                current(),
                "No root-level keymap overrides have been configured.",
            ),
        ),
    });

    let unbound_rows = rows
        .iter()
        .filter(|row| row.is_unbound())
        .collect::<Vec<_>>();
    tabs.push(SelectionTab {
        id: KEYMAP_UNBOUND_TAB_ID.to_string(),
        label: tr_with(current(), "Unbound ({0})", &[&unbound_count.to_string()]).to_string(),
        header: keymap_header(
            tr(current(), "Actions without an active shortcut.").to_string(),
            action_count_line(unbound_count),
        ),
        items: keymap_selection_items(
            unbound_rows,
            tr(current(), "No unbound shortcuts"),
            tr(
                current(),
                "Every configurable action currently has a shortcut.",
            ),
        ),
    });

    for tab in keymap_context_tabs() {
        let tab_rows = rows
            .iter()
            .filter(|row| tab.contexts.contains(&row.context))
            .collect::<Vec<_>>();
        let count = tab_rows.len();
        tabs.push(SelectionTab {
            id: tab.id.to_string(),
            label: tab.label.to_string(),
            header: keymap_header(tab.description.to_string(), action_count_line(count)),
            items: keymap_selection_items(
                tab_rows,
                tr(current(), "No shortcuts in this group"),
                tr(
                    current(),
                    "No configurable actions are available in this group.",
                ),
            ),
        });
    }
    tabs.push(keymap_debug_tab());

    SelectionViewParams {
        view_id: Some(KEYMAP_PICKER_VIEW_ID),
        header: Box::new(()),
        footer_hint: Some(keymap_picker_hint_line()),
        tab_footer_hints: vec![(KEYMAP_DEBUG_TAB_ID.to_string(), keymap_debug_hint_line())],
        tabs,
        initial_tab_id: Some(KEYMAP_ALL_TAB_ID.to_string()),
        is_searchable: true,
        search_placeholder: Some(tr(current(), "Type to search shortcuts").to_string()),
        col_width_mode: ColumnWidthMode::AutoAllRows,
        row_display: SelectionRowDisplay::SingleLine,
        name_column_width,
        initial_selected_idx,
        ..Default::default()
    }
}

fn keymap_debug_tab() -> SelectionTab {
    SelectionTab {
        id: KEYMAP_DEBUG_TAB_ID.to_string(),
        label: tr(current(), "Debug").to_string(),
        header: keymap_header(
            tr(current(), "Inspect keypresses from your terminal.").to_string(),
            tr(current(), "See the key Codex detects and any shortcuts assigned to it.").to_string(),
        ),
        items: vec![SelectionItem {
            name: tr(current(), "Inspect keypresses").to_string(),
            description: Some(
                tr(current(), "Press Enter to start. Then press any key to inspect it; Ctrl+C exits.")
                    .to_string(),
            ),
            selected_description: Some(
                tr(current(), "Open a live inspector that shows the detected key, config key, and matching actions.")
                    .to_string(),
            ),
            actions: vec![Box::new(|tx| {
                tx.send(AppEvent::OpenKeymapDebug);
            })],
            search_value: Some("debug inspect keypress key terminal detected actions".to_string()),
            ..Default::default()
        }],
    }
}

fn build_keymap_rows(
    runtime_keymap: &RuntimeKeymap,
    keymap_config: &TuiKeymap,
    action_filter: KeymapActionFilter,
) -> Vec<KeymapActionRow> {
    keymap_actions()
        .iter()
        .copied()
        .filter(|descriptor| descriptor.is_visible(action_filter))
        .map(|descriptor| KeymapActionRow {
            context: descriptor.context,
            context_label: descriptor.context_label,
            action: descriptor.action,
            label: action_label(descriptor.action),
            description: descriptor.description,
            binding_summary: format_action_binding_summary(
                runtime_keymap,
                descriptor.context,
                descriptor.action,
            ),
            custom_binding: has_custom_binding(
                keymap_config,
                descriptor.context,
                descriptor.action,
            )
            .unwrap_or(/*default*/ false),
        })
        .collect()
}

fn keymap_common_rows(rows: &[KeymapActionRow]) -> Vec<&KeymapActionRow> {
    KEYMAP_COMMON_ACTIONS
        .iter()
        .filter_map(|(context, action)| {
            rows.iter()
                .find(|row| row.context == *context && row.action == *action)
        })
        .collect()
}

fn keymap_selection_items<'a>(
    rows: impl IntoIterator<Item = &'a KeymapActionRow>,
    empty_name: &str,
    empty_description: &str,
) -> Vec<SelectionItem> {
    let items = rows
        .into_iter()
        .map(keymap_selection_item)
        .collect::<Vec<_>>();
    if items.is_empty() {
        return vec![SelectionItem {
            name: empty_name.to_string(),
            description: Some(empty_description.to_string()),
            is_disabled: true,
            ..Default::default()
        }];
    }

    items
}

fn keymap_selection_item(row: &KeymapActionRow) -> SelectionItem {
    let context = row.context.to_string();
    let action = row.action.to_string();
    let source = if row.custom_binding {
        "Custom"
    } else {
        "Default"
    };
    let search_value = format!(
        "{} {} {} {} {} {}",
        row.context_label, row.action, row.label, row.description, row.binding_summary, source
    );

    SelectionItem {
        name: row.label.clone(),
        name_prefix_spans: keymap_row_prefix(row),
        description: Some(row.binding_summary.clone()),
        actions: vec![Box::new(move |tx| {
            tx.send(AppEvent::OpenKeymapActionMenu {
                context: context.clone(),
                action: action.clone(),
            });
        })],
        search_value: Some(search_value),
        ..Default::default()
    }
}

fn keymap_row_prefix(row: &KeymapActionRow) -> Vec<Span<'static>> {
    let indicator = if row.custom_binding {
        "*".set_style(accent_style())
    } else if row.is_unbound() {
        "-".dim()
    } else {
        " ".into()
    };

    vec![
        format!(
            "{:<width$} ",
            row.context_label,
            width = KEYMAP_CONTEXT_LABEL_WIDTH
        )
        .dim(),
        indicator,
        " ".dim(),
    ]
}

fn keymap_header(description: String, summary: String) -> Box<dyn Renderable> {
    let mut header = ColumnRenderable::new();
    header.push(Line::from(tr(current(), "Keymap").bold()));
    header.push(Line::from(description.dim()));
    header.push(Line::from(summary.dim()));
    Box::new(header)
}

fn action_count_line(count: usize) -> String {
    match count {
        1 => tr(current(), "1 action.").to_string(),
        _ => tr_with(current(), "{0} actions.", &[&count.to_string()]).to_string(),
    }
}

fn keymap_picker_hint_line() -> Line<'static> {
    let style = accent_style();
    Line::from(vec![
        "left/right".set_style(style),
        tr(current(), " group · ").dim(),
        "enter".set_style(style),
        tr(current(), " edit shortcut · ").dim(),
        "*".set_style(style),
        tr(current(), " custom · ").dim(),
        "-".set_style(style),
        tr(current(), " unbound · ").dim(),
        "esc".set_style(style),
        tr(current(), " close").dim(),
    ])
}

fn keymap_debug_hint_line() -> Line<'static> {
    let style = accent_style();
    Line::from(vec![
        "enter".set_style(style),
        tr(current(), " start inspector · ").dim(),
        "esc".set_style(style),
        tr(current(), " close").dim(),
    ])
}
