//! Shared UI constants for layout and alignment within the TUI.

use codex_i18n::current;
use codex_i18n::tr;
/// Width (in terminal columns) reserved for the left gutter/prefix used by
/// live cells and aligned widgets.
///
/// Semantics:
/// - Chat composer reserves this many columns for the left border + padding.
/// - Status indicator lines begin with this many spaces for alignment.
/// - User history lines account for this many columns (e.g., "▌ ") when wrapping.
pub(crate) const LIVE_PREFIX_COLS: u16 = 2;
pub(crate) const FOOTER_INDENT_COLS: usize = LIVE_PREFIX_COLS as usize;
/// Keycap hint that advertises the transcript shortcut next to a truncated
/// output block.
///
/// A function rather than a constant because `tr` resolves at render time: the
/// published language can change while the TUI is open, and a `const` would
/// freeze whichever language happened to be active when the crate loaded.
pub(crate) fn transcript_hint() -> &'static str {
    tr(current(), "ctrl + t to view transcript")
}
