use pretty_assertions::assert_eq;

use crate::Lang;
use crate::tr;

#[test]
fn english_returns_the_key_verbatim() {
    assert_eq!(tr(Lang::En, "Show this help"), "Show this help");
}

#[test]
fn english_is_byte_identical_for_awkward_strings() {
    // Strings that a naive implementation could mangle: leading/trailing
    // whitespace, embedded newlines, ANSI escapes and non-ASCII text.
    for key in [
        "",
        " ",
        "  padded  ",
        "line one\nline two",
        "\u{1b}[1mbold\u{1b}[0m",
        "↑ to manage attachments",
        "100% 完成",
    ] {
        assert_eq!(tr(Lang::En, key), key);
    }
}

#[test]
fn chinese_falls_back_to_english_for_untranslated_keys() {
    // The dictionary is partial by design, so a lookup that misses must fall
    // back rather than return a placeholder or an empty string.
    assert_eq!(tr(Lang::Zh, "Show this help"), "Show this help");
    assert_eq!(tr(Lang::Zh, ""), "");
}

#[test]
fn chinese_translates_the_keys_the_dictionary_carries() {
    // The other half of the contract: a key that *is* in the dictionary must
    // come back translated. These three are the footer hints, the first
    // vertical slice of the rollout.
    assert_eq!(tr(Lang::Zh, " for agents"), " 切换智能体");
    assert_eq!(tr(Lang::Zh, " for shortcuts"), " 查看快捷键");
    assert_eq!(tr(Lang::Zh, " to queue message"), " 排队发送消息");
}

#[test]
fn english_ignores_the_dictionary_entirely() {
    // Byte-identical English output is the property the snapshots rest on: a
    // translated key must still render as its English original under `En`.
    for key in [" for agents", " for shortcuts", " to queue message"] {
        assert_eq!(tr(Lang::En, key), key);
    }
}
