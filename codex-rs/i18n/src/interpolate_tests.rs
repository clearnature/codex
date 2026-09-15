//! Tests for `interpolate.rs`.
//!
//! The dictionary is not consulted for most of these: the substitution rule is
//! the contract, and `tr_with` is only the dictionary lookup plus one call to
//! it. `interpolated_translations_keep_their_placeholders` is the exception --
//! it walks the shipped dictionary and is the guard a new entry has to pass.

use super::placeholders;
use super::substitute;
use super::tr_with;
use crate::Lang;
use crate::dict_zh::ENTRIES;

#[test]
fn english_substitutes_into_the_english_original() {
    // The property that keeps pre-i18n behaviour: English is the template.
    assert_eq!(
        tr_with(Lang::En, "Deleted {0} files", &["3"]),
        "Deleted 3 files"
    );
    assert_eq!(
        tr_with(Lang::En, "Deleted {0} files in {1}", &["3", "src/"]),
        "Deleted 3 files in src/"
    );
}

#[test]
fn a_template_without_placeholders_ignores_its_arguments() {
    assert_eq!(tr_with(Lang::En, "Ready", &["unused"]), "Ready");
    assert_eq!(tr_with(Lang::Zh, "Ready", &["unused"]), "Ready");
}

#[test]
fn a_placeholder_without_an_argument_is_left_verbatim() {
    // A translator (or a call site) that references a value nobody passed must
    // not panic and must not silently drop the token.
    assert_eq!(substitute("Deleted {0} files", &[]), "Deleted {0} files");
    assert_eq!(substitute("{0} and {1}", &["a"]), "a and {1}");
}

#[test]
fn non_placeholder_braces_are_copied_through() {
    assert_eq!(substitute("{not a number}", &["x"]), "{not a number}");
    assert_eq!(substitute("unclosed {0", &["x"]), "unclosed {0");
    assert_eq!(substitute("{", &["x"]), "{");
    assert_eq!(substitute("{}", &["x"]), "{}");
}

#[test]
fn a_placeholder_may_repeat() {
    assert_eq!(substitute("{0}/{0} = 1", &["7"]), "7/7 = 1");
}

#[test]
fn placeholders_are_reported_in_order_without_duplicates() {
    assert_eq!(placeholders("Deleted {0} files in {1}"), vec![0, 1]);
    assert_eq!(placeholders("{1} then {0} then {1}"), vec![1, 0]);
    assert_eq!(placeholders("no placeholders"), Vec::<usize>::new());
    assert_eq!(placeholders("{name}"), Vec::<usize>::new());
}

#[test]
fn interpolated_translations_keep_their_placeholders() {
    // Parity: every shipped entry must use exactly the placeholders its English
    // key uses. This is the check a new entry has to pass, and it is the reason
    // `placeholders` is part of the public API at all.
    for (key, translated) in ENTRIES {
        // Sets, not sequences: a translation is free to reorder the values (that
        // is the whole reason `tr_with` substitutes after the lookup), it just
        // may not drop or invent one. Our first draft compared the ordered lists
        // and rejected a perfectly good Chinese sentence for putting `{0}` last.
        let mut expected = placeholders(key);
        let mut actual = placeholders(translated);
        expected.sort_unstable();
        actual.sort_unstable();
        assert_eq!(
            actual, expected,
            "entry {key:?} -> {translated:?} does not keep the same placeholders"
        );
    }
}

#[test]
fn a_translated_template_interpolates_after_translation() {
    // With a dictionary hit, substitution happens on the translation, which is
    // what lets a language reorder values. `Ready` is untranslated, so this
    // exercises the fallback path plus substitution; the dictionary-backed path
    // is covered by the parity test and by the footer entries downstream.
    assert_eq!(tr_with(Lang::Zh, "Ready {0}", &["now"]), "Ready now");
}
