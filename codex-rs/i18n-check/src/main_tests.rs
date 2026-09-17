use std::collections::BTreeMap;

use pretty_assertions::assert_eq;

use crate::asset_row_drift;

use crate::duplicate_keys;
use crate::extract_label_literals;
use crate::extract_tr_calls;
use crate::named_placeholder_hits;
use crate::named_placeholders;
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
fn finds_a_named_placeholder_but_not_an_index_or_prose() {
    // The engine substitutes `{0}`, `{1}`, ... and copies anything else through
    // verbatim, so `{label}` is the shape that reaches the screen unchanged.
    // `{not a number}` and `{}` are prose/idioms, not placeholders -- flagging
    // them would train the reader to ignore the report.
    assert_eq!(named_placeholders("     {label}: <empty>"), vec!["label"]);
    assert_eq!(named_placeholders("{action}/{action}"), vec!["action"]);
    assert_eq!(named_placeholders("Field {0}/{1}"), Vec::<String>::new());
    assert_eq!(named_placeholders("{not a number}"), Vec::<String>::new());
    assert_eq!(named_placeholders("{}"), Vec::<String>::new());
    assert_eq!(named_placeholders("no placeholders"), Vec::<String>::new());
}

#[test]
fn reports_a_named_placeholder_on_either_side_of_the_dictionary() {
    let pairs = vec![
        ("Fine {0}".to_string(), "没问题 {0}".to_string()),
        (
            "     {label}: <empty>".to_string(),
            "     {0}: <空>".to_string(),
        ),
        ("Bad".to_string(), "坏 {reason}".to_string()),
    ];
    let hits = named_placeholder_hits(&pairs, &BTreeMap::new());
    // The hit is keyed by the *template* that carries the token, so a broken
    // translation reports its own text (with the origin as the site) rather
    // than the English key it belongs to.
    assert_eq!(
        hits.keys().cloned().collect::<Vec<_>>(),
        vec![
            "     {label}: <empty>".to_string(),
            "坏 {reason}".to_string()
        ]
    );
    assert_eq!(
        hits["坏 {reason}"].1,
        vec!["dictionary translation".to_string()]
    );
}

#[test]
fn reports_a_call_site_that_renders_a_named_placeholder() {
    let calls = extract_tr_calls(r#"tr_with(current(), "     {label}: <empty>", &[label])"#);
    assert_eq!(calls.len(), 1);
    let used: BTreeMap<String, Vec<String>> =
        BTreeMap::from([(calls[0].key.clone(), vec!["src/a.rs:1".to_string()])]);
    let hits = named_placeholder_hits(&[], &used);
    assert_eq!(hits[&calls[0].key].1, vec!["src/a.rs:1".to_string()]);
}

#[test]
fn reports_a_key_declared_twice_with_both_translations() {
    // The map keeps the last entry, so the earlier translation is dead text --
    // and no count in the report changes when a batch re-translates a key.
    let pairs = vec![
        ("Quit".to_string(), "退出".to_string()),
        ("Ready".to_string(), "就绪".to_string()),
        ("Ready".to_string(), "准备好了".to_string()),
    ];
    assert_eq!(
        duplicate_keys(&pairs),
        vec![("Ready", vec!["就绪", "准备好了"])]
    );
    assert_eq!(duplicate_keys(&pairs[..1]), Vec::<(&str, Vec<&str>)>::new());
}

#[test]
fn reports_asset_rows_missing_from_either_side() {
    // The rows are the identity of a template (connector_id + server + tool
    // title); the template text is what a translation may change.
    let english = vec!["c1|srv|title_a".to_string(), "c1|srv|title_b".to_string()];
    let same = english.clone();
    assert_eq!(asset_row_drift(&english, &same), Vec::<String>::new());

    let short = vec!["c1|srv|title_a".to_string()];
    assert_eq!(
        asset_row_drift(&english, &short),
        vec!["missing from the translation: c1|srv|title_b".to_string()]
    );

    let extra = vec![
        "c1|srv|title_a".to_string(),
        "c1|srv|title_b".to_string(),
        "c2|srv|title_c".to_string(),
    ];
    assert_eq!(
        asset_row_drift(&english, &extra),
        vec!["not in the English asset: c2|srv|title_c".to_string()]
    );
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
