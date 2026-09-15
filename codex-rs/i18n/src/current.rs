//! The language a front end has selected for this process.
//!
//! Rendering sites are deep inside widget trees (`bottom_pane/footer.rs` and
//! the rest of `tui`), and threading a `Lang` argument down to each of them
//! would change hundreds of signatures for a value that is fixed once per
//! invocation. The reference implementation solves the same problem the same
//! way -- `qwen-code`'s `packages/cli/src/i18n/index.ts` keeps one module-level
//! language that `setLanguage` publishes and `t` reads back -- so this crate
//! does the same, with two differences that fit the constraint the reference
//! does not have:
//!
//! * the default is [`Lang::En`], never "unset", so a rendering path that runs
//!   before a front end publishes a language still renders English rather than
//!   a placeholder;
//! * publishing is idempotent and lock-free, so several front ends (or a
//!   restart inside one process) can call [`set_current`] without a race.
//!
//! The value is a plain `AtomicU8` keyed by the small encoding below rather
//! than an `OnceLock`, because "whoever gets there first wins" is exactly the
//! failure mode to avoid when the command line and the configuration are read
//! at different moments of startup.

use crate::lang::Lang;
use std::sync::atomic::AtomicU8;
use std::sync::atomic::Ordering;

/// Encoding of [`Lang::En`].
const EN: u8 = 0;

/// Encoding of [`Lang::Zh`].
const ZH: u8 = 1;

/// The language for this process. English until a front end says otherwise.
static CURRENT: AtomicU8 = AtomicU8::new(EN);

/// Encodes a [`Lang`] as the byte stored in [`CURRENT`].
fn encode(lang: Lang) -> u8 {
    match lang {
        Lang::En => EN,
        Lang::Zh => ZH,
    }
}

/// Decodes a byte stored in [`CURRENT`]. Any unknown byte means English, which
/// cannot happen through [`set_current`] and is not worth a panic if it ever
/// did through a future edit.
fn decode(raw: u8) -> Lang {
    match raw {
        ZH => Lang::Zh,
        _ => Lang::En,
    }
}

/// The language rendering sites should use right now.
#[must_use]
pub fn current() -> Lang {
    decode(CURRENT.load(Ordering::Relaxed))
}

/// Publishes the language for this process and returns the previous one.
///
/// Idempotent: calling it twice with the same value is fine, and the second
/// call returns the same value it was given. The previous value is returned
/// instead of `()` so that a caller which needs to restore state -- a test, or
/// a front end re-resolving after loading configuration -- can.
pub fn set_current(lang: Lang) -> Lang {
    decode(CURRENT.swap(encode(lang), Ordering::Relaxed))
}

#[cfg(test)]
#[path = "current_tests.rs"]
mod tests;
