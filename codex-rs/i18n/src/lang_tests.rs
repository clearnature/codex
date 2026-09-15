use pretty_assertions::assert_eq;

use crate::Lang;
use crate::parse_lang;

#[test]
fn resolves_the_documented_spellings_to_chinese() {
    for raw in ["zh", "cn", "zh-CN", "zh-hans", "中文"] {
        assert_eq!(parse_lang(raw), Lang::Zh, "input: {raw:?}");
    }
}

#[test]
fn matching_is_case_insensitive_and_treats_underscores_as_dashes() {
    for raw in ["ZH", " Zh ", "zh_cn", "ZH_CN", "ZH-HANS"] {
        assert_eq!(parse_lang(raw), Lang::Zh, "input: {raw:?}");
    }
}

#[test]
fn falls_back_to_english_for_unknown_locales() {
    // An unknown locale is never an error and never yields a half-translated
    // UI, so everything here must resolve to English.
    //
    // `C` and `POSIX` are what `sys-locale` returns when `LC_ALL` / `LC_CTYPE`
    // force the C locale -- which codex's own unified-exec environment does for
    // child shells. `ja-JP` and `zh-TW` are real tag shapes it returns for
    // languages this stage does not translate.
    for raw in [
        "",
        "C",
        "POSIX",
        "en",
        "en-US",
        "zh-Hant",
        "zh-TW",
        "fr",
        "de-DE",
        "ja-JP",
        "日本語",
        "xx",
    ] {
        assert_eq!(parse_lang(raw), Lang::En, "input: {raw:?}");
    }
}
