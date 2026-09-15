//! Tests for the process-wide current language.
//!
//! `current()` is the one piece of mutable state in this crate, so the
//! assertions about it live in a single test function: Rust runs tests in
//! parallel threads inside one process, and splitting these into several
//! `#[test]`s would let one observe another's half-finished transition. No
//! other test in the crate reads `current()`, so this stays the only observer.

use super::current;
use super::set_current;
use crate::Lang;
use crate::tr;

#[test]
fn the_current_language_is_english_by_default_and_can_be_republished() {
    // Normalise first so the test does not depend on whether some other code in
    // this binary published a language before it. The value it returns is not
    // asserted -- only the transitions that follow are.
    let _ = set_current(Lang::En);

    assert_eq!(current(), Lang::En);
    assert_eq!(tr(current(), "Show this help"), "Show this help");

    // Publishing Chinese switches what rendering sites see...
    assert_eq!(set_current(Lang::Zh), Lang::En);
    assert_eq!(current(), Lang::Zh);
    // ...and with an empty dictionary the lookup still degrades to English
    // rather than to a placeholder.
    assert_eq!(tr(current(), "Show this help"), "Show this help");

    // Republishing is idempotent and reports the value it was given.
    assert_eq!(set_current(Lang::Zh), Lang::Zh);
    assert_eq!(current(), Lang::Zh);

    // Restoring is what lets a front end re-resolve after loading config.
    assert_eq!(set_current(Lang::En), Lang::Zh);
    assert_eq!(current(), Lang::En);
    assert_eq!(tr(current(), "Show this help"), "Show this help");
}
