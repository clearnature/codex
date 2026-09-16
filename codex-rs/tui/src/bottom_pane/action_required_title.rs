use super::TerminalTitleItem;
use codex_i18n::current;
use codex_i18n::tr;

/// Prefix for the "action required" terminal-title preview.
///
/// A function rather than a constant because `tr` resolves at render time: the
/// published language can change while the TUI is open.
pub(crate) fn action_required_preview_prefix() -> &'static str {
    tr(current(), "[ ! ] Action Required")
}

pub(crate) fn build_action_required_title_text<I, F>(
    prefix: &str,
    items: I,
    excluded_items: &[TerminalTitleItem],
    mut value_for: F,
) -> String
where
    I: IntoIterator<Item = TerminalTitleItem>,
    F: FnMut(TerminalTitleItem) -> Option<String>,
{
    let mut parts = vec![prefix.to_string()];
    for item in items {
        if item == TerminalTitleItem::Spinner || excluded_items.contains(&item) {
            continue;
        }
        if let Some(value) = value_for(item) {
            parts.push(value);
        }
    }
    parts.join(" | ")
}
