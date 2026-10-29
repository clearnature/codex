// Aggregates all former standalone integration tests as modules.
#[cfg(unix)]
mod daemon_compatibility;
#[cfg(unix)]
mod directory_trust;
#[cfg(unix)]
mod focus_palette;
#[cfg(unix)]
mod provider_defaults;
#[cfg(unix)]
mod reconnect;
mod resize_reflow;
#[cfg(unix)]
mod screen_reader;
mod status_indicator;
mod vt100_history;
mod vt100_live_commit;
#[cfg(unix)]
mod worktree_stack;

// `tests/suite` asserts on English copy only. Rendered text goes through
// `tr`, so pin the whole test process to English before any screen is built
// (a zh-CN developer host would otherwise leak into these assertions).
pub(crate) mod language_pin {
    // `tests/suite` asserts on English copy only. Rendered text goes through
    // `tr`, so pin the whole test process to English before any screen is
    // built (a zh-CN developer host would otherwise leak into assertions).
    #[ctor::ctor]
    unsafe fn pin_english() {
        codex_i18n::set_current(codex_i18n::Lang::En);
    }
}
