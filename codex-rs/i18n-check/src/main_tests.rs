use pretty_assertions::assert_eq;

use crate::extract_label_literals;
use crate::extract_tr_calls;
use crate::read_dictionary_pairs;
use crate::spacing_violations;

#[test]
fn flags_a_space_between_cjk_and_latin_but_not_a_positional_one() {
    // The glossary asks for `启用{0}并记住此选择`, not `启用 {0} 并记住此选择`. The
    // exception matters just as much: `" to move"` is appended after a keybinding
    // and its translation starts with that space on purpose, so a check that
    // flagged leading/trailing spaces would be wrong about the layout.
    let pairs = vec![
        ("Quit".to_string(), "退出".to_string()),
        ("Open".to_string(), "打开 Codex".to_string()),
        (" to move".to_string(), " 移动".to_string()),
        ("Context".to_string(), "剩余上下文 100%".to_string()),
    ];
    assert_eq!(
        spacing_violations(&pairs)
            .into_iter()
            .map(|(key, _)| key)
            .collect::<Vec<_>>(),
        vec!["Open", "Context"]
    );
}

#[test]
fn finds_a_template_rendered_with_tr_with() {
    // The interpolation form has the same `(lang, key, args)` shape, so the same
    // rule applies. Missing it made a string that is rendered on every keystroke
    // look unused -- and "unused" is the report that invites deleting it.
    let calls = extract_tr_calls(r#"tr_with(current(), "{0}% context left", &["7"])"#);
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].key, "{0}% context left");
}

#[test]
fn finds_a_label_literal() {
    let calls = extract_label_literals(r#"        label: " for commands","#);
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].key, " for commands");
}

#[test]
fn ignores_labels_that_are_not_literals() {
    assert_eq!(extract_label_literals("label: self.name"), vec![]);
    assert_eq!(extract_label_literals("labelled: \"x\""), vec![]);
    assert_eq!(extract_label_literals("let label = 3;"), vec![]);
}

#[test]
fn ignores_an_empty_label() {
    // An empty label is a layout placeholder, not translatable text.
    assert_eq!(extract_label_literals("        label: \"\","), vec![]);
}

#[test]
fn does_not_confuse_the_word_label_inside_a_string() {
    assert_eq!(
        extract_label_literals(r#"let s = "label: \" for commands\"";"#),
        vec![]
    );
}

#[test]
fn finds_a_plain_call() {
    let calls = extract_tr_calls(r#"let label = tr(lang, " for agents");"#);
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].key, " for agents");
}

#[test]
fn ignores_the_tr_inside_as_ptr() {
    // `as_ptr(` contains the substring `tr(`; a naive scan reports a phantom key.
    let calls = extract_tr_calls("let p = value.as_ptr();");
    assert_eq!(calls, vec![]);
}

#[test]
fn finds_a_multi_line_call_and_ignores_the_lang_argument() {
    let source = "let line = tr(\n    lang,\n    \" to queue message\",\n);";
    let calls = extract_tr_calls(source);
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].key, " to queue message");
    assert_eq!(calls[0].line, 1);
}

#[test]
fn reads_raw_strings_verbatim() {
    let calls = extract_tr_calls(r##"tr(lang, r#"raw "quoted" text"#)"##);
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].key, r#"raw "quoted" text"#);
}

#[test]
fn decodes_escapes_so_keys_compare_equal_to_the_dictionary_form() {
    let calls = extract_tr_calls(r#"tr(lang, "line one\nline two")"#);
    assert_eq!(calls.len(), 1);
    assert_eq!(calls[0].key, "line one\nline two");
}

#[test]
fn skips_calls_whose_first_argument_is_not_a_literal() {
    assert_eq!(extract_tr_calls("tr(lang, key)"), vec![]);
}

#[test]
fn ignores_commented_out_calls() {
    assert_eq!(extract_tr_calls(r#"// tr(lang, " commented ")"#), vec![]);
    assert_eq!(extract_tr_calls(r#"/* tr(lang, " blocked ") */"#), vec![]);
}

#[test]
fn reads_dictionary_entries_in_source_order() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("dict_zh.rs");
    std::fs::write(
        &path,
        r#"
static ENTRIES: &[(&str, &str)] = &[
    ("Show this help", "显示此帮助"),
    ("Quit", "退出"),
];
"#,
    )
    .expect("write");
    let entries = read_dictionary_pairs(&path).expect("read");
    assert_eq!(
        entries.into_iter().map(|(key, _)| key).collect::<Vec<_>>(),
        vec!["Show this help".to_string(), "Quit".to_string()]
    );
}

#[test]
fn an_empty_dictionary_yields_no_entries() {
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("dict_zh.rs");
    std::fs::write(&path, "static ENTRIES: &[(&str, &str)] = &[];\n").expect("write");
    assert_eq!(read_dictionary_pairs(&path).expect("read").len(), 0);
}

#[test]
fn reads_entries_after_a_key_that_contains_a_semicolon() {
    // The end of the array must not be found by a plain "first `;`" search:
    // several English originals contain a semicolon ("… only; omitted when …"),
    // and stopping there silently drops every entry after it -- which is exactly
    // the drift this tool exists to catch, so it must not be introduced by the
    // tool itself. This regressed once; the assertion keeps it from coming back.
    let dir = tempfile::tempdir().expect("tempdir");
    let path = dir.path().join("dict_zh.rs");
    std::fs::write(
        &path,
        r#"
static ENTRIES: &[(&str, &str)] = &[
    ("only; omitted when unavailable", "仅在不可用时省略"),
    ("Quit", "退出"),
];
"#,
    )
    .expect("write");
    let entries = read_dictionary_pairs(&path).expect("read");
    assert_eq!(
        entries.into_iter().map(|(key, _)| key).collect::<Vec<_>>(),
        vec![
            "only; omitted when unavailable".to_string(),
            "Quit".to_string()
        ]
    );
}
