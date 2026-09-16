//! Shared popup-related constants for bottom pane widgets.

use codex_i18n::current;
use codex_i18n::tr;
use codex_i18n::tr_with;
use ratatui::text::Line;

use crate::key_hint;
use crate::key_hint::ShortcutHint;
use crate::keymap::ListAction;
use crate::keymap::ListKeymap;
use crossterm::event::KeyCode;

/// Maximum number of rows any popup should attempt to display.
/// Keep this consistent across all popups for a uniform feel.
pub(crate) const MAX_POPUP_ROWS: usize = 8;

/// Standard footer hint text used by popups.
pub(crate) fn standard_popup_hint_line() -> Line<'static> {
    Line::from(vec![
        tr(current(), "Press ").into(),
        key_hint::plain(KeyCode::Enter).into(),
        tr(current(), " to confirm or ").into(),
        key_hint::plain(KeyCode::Esc).into(),
        tr(current(), " to go back").into(),
    ])
}

pub(crate) fn standard_popup_hint_line_for_keymap(list_keymap: &ListKeymap) -> Line<'static> {
    accept_cancel_hint_line(
        list_keymap.primary_hint(ListAction::Accept),
        tr(current(), "to confirm"),
        list_keymap.primary_hint(ListAction::Cancel),
        tr(current(), "to go back"),
    )
}

pub(crate) fn accept_cancel_hint_line(
    accept: Option<ShortcutHint>,
    accept_label: &'static str,
    cancel: Option<ShortcutHint>,
    cancel_label: &'static str,
) -> Line<'static> {
    match (accept, cancel) {
        (Some(accept), Some(cancel)) => Line::from(vec![
            tr(current(), "Press ").into(),
            accept.into(),
            tr_with(current(), " {0} or ", &[accept_label]).into(),
            cancel.into(),
            format!(" {cancel_label}").into(),
        ]),
        (Some(accept), None) => Line::from(vec![
            tr(current(), "Press ").into(),
            accept.into(),
            format!(" {accept_label}").into(),
        ]),
        (None, Some(cancel)) => Line::from(vec![
            tr(current(), "Press ").into(),
            cancel.into(),
            format!(" {cancel_label}").into(),
        ]),
        (None, None) => Line::from(""),
    }
}
