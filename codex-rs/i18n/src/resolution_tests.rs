//! Tests for the precedence chain in `resolution.rs`.
//!
//! Every test drives `resolve` with all four sources given as data, so the
//! suite never depends on the locale of the machine it runs on and never has to
//! mutate the environment (which would be visible to tests running in parallel).

use super::Env;
use super::resolve;
use crate::Lang;

/// Builds an [`Env`] from two optional variables.
fn env(lc_all: Option<&str>, lang: Option<&str>) -> Env {
    Env {
        lc_all: lc_all.map(str::to_owned),
        lang: lang.map(str::to_owned),
    }
}

/// The empty environment: no `LC_ALL`, no `LANG`.
fn no_env() -> Env {
    Env::default()
}

#[test]
fn cli_lang_beats_config_environment_and_system() {
    assert_eq!(
        resolve(
            Some("zh"),
            Some("en"),
            &env(Some("en_US.UTF-8"), Some("en_US.UTF-8")),
            Some("en_US.UTF-8"),
        ),
        Lang::Zh
    );
}

#[test]
fn config_lang_beats_environment_and_system() {
    assert_eq!(
        resolve(
            None,
            Some("zh-CN"),
            &env(Some("en_US.UTF-8"), Some("en_US.UTF-8")),
            Some("en_US.UTF-8"),
        ),
        Lang::Zh
    );
}

#[test]
fn lc_all_beats_lang() {
    assert_eq!(
        resolve(
            None,
            None,
            &env(Some("zh_CN.UTF-8"), Some("en_US.UTF-8")),
            None
        ),
        Lang::Zh
    );
}

#[test]
fn lang_beats_the_system_locale() {
    assert_eq!(
        resolve(
            None,
            None,
            &env(None, Some("zh_CN.UTF-8")),
            Some("en_US.UTF-8")
        ),
        Lang::Zh
    );
}

#[test]
fn the_system_locale_is_the_last_resort() {
    assert_eq!(
        resolve(None, None, &no_env(), Some("zh_CN.UTF-8")),
        Lang::Zh
    );
}

#[test]
fn no_source_at_all_resolves_to_english() {
    assert_eq!(resolve(None, None, &no_env(), None), Lang::En);
}

#[test]
fn an_explicit_english_request_beats_a_chinese_lower_source() {
    // Presence decides, not whether the value happens to be translated: asking
    // for English on the command line must not be overridden by config.toml.
    assert_eq!(
        resolve(
            Some("en"),
            Some("zh"),
            &env(Some("zh_CN.UTF-8"), None),
            Some("zh_CN.UTF-8")
        ),
        Lang::En
    );
}

#[test]
fn an_unknown_language_degrades_to_english_rather_than_erroring() {
    assert_eq!(resolve(Some("fr"), None, &no_env(), None), Lang::En);
}

#[test]
fn empty_and_whitespace_values_do_not_mask_lower_sources() {
    assert_eq!(
        resolve(None, Some("  "), &env(Some(""), Some("zh")), None),
        Lang::Zh
    );
}

#[test]
fn underscore_and_case_variants_resolve_like_tags() {
    assert_eq!(
        resolve(None, None, &env(Some("ZH_cn"), None), None),
        Lang::Zh
    );
    assert_eq!(
        resolve(None, None, &env(Some("zh_CN.UTF-8"), None), None),
        Lang::Zh
    );
}

#[test]
fn the_posix_c_locale_is_english() {
    // `sys-locale` reports `C` or `POSIX` for a process with no locale set.
    assert_eq!(resolve(None, None, &no_env(), Some("C")), Lang::En);
    assert_eq!(resolve(None, None, &no_env(), Some("POSIX")), Lang::En);
}

#[test]
fn env_first_prefers_lc_all_over_lang() {
    assert_eq!(env(Some("zh"), Some("en")).first(), Some("zh"));
    assert_eq!(env(None, Some("en")).first(), Some("en"));
    assert_eq!(no_env().first(), None);
}

#[test]
fn an_empty_environment_selects_nothing() {
    assert_eq!(env(Some(""), Some("")).first(), None);
}
