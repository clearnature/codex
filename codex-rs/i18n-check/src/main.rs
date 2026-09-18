//! Drift detection for the `codex-i18n` dictionary.
//!
//! H3 of `docs/plan/i18n-verification.md` asks whether drift -- a string being
//! rendered without a translation, or an entry whose string no longer exists --
//! can be found automatically. Its refutation condition is "the equivalent of
//! `extractUsedKeys` cannot be built", so this binary is the prototype, and it
//! covers the three checks the plan names: missing, unused and coverage, plus
//! the invariants the same scan can see for free and those three are blind
//! to: a key declared twice (the map keeps the last entry, so the earlier
//! translations are dead text) and a *named* placeholder such as `{label}`
//! (the engine substitutes positional `{N}` only, so the token reaches the
//! screen verbatim -- see `codex-rs/i18n/src/interpolate.rs`), plus asset
//! reconciliation: a translated asset must carry exactly the same rows as the
//! English one, because nothing else can compare two JSON files.
//!
//! It reads two sources:
//!   * every `tr(..)` / `tr_with(..)` call in the workspace, taking the first
//!     literal argument as the key the code actually renders. The scan is
//!     lexical but not naive: it walks comments, string literals and char
//!     literals, so the `tr(` inside `as_ptr(` is not mistaken for a call, and
//!     multi-line calls still work.
//!   * the `ENTRIES` array in `codex-rs/i18n/src/dict_zh.rs`.
//!
//! Keys that never appear as a literal at a call site are counted too, but
//! reported separately as *bound* keys: a shortcut descriptor keeps its English
//! text in a `label` field and renders it as `tr(current(), self.label)`, so the
//! literal lives in a table rather than in the call. Treating those as unused
//! would be the wrong error in the dangerous direction -- the report's "unused"
//! list is what invites deleting a translation that is on screen constantly --
//! so the rule is deliberately generous and transparent instead.
//!
//! Usage:
//!   cargo run -p codex-i18n-check
//!   cargo run -p codex-i18n-check -- --root /path/to/repo
//!
//! Exits non-zero when anything drifted, so it can gate CI.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::ExitCode;

const HELP: &str = "\
codex-i18n-check -- detect drift between rendered strings and the zh dictionary

Usage:
  codex-i18n-check [--root <repo root>]

Checks reported:
  missing      keys rendered in code but absent from the dictionary
  unused       dictionary entries no longer rendered anywhere
  coverage     share of rendered keys that have a translation
  duplicate    dictionary keys declared more than once (the last one wins)
  placeholder  named placeholders (`{label}`), which the engine copies verbatim
  nested       tr/tr_with inside another tr/tr_with (double translation)
  asset        a translated asset whose rows differ from the English one
";

/// One rendered key, with the place it was rendered.
#[derive(Debug, PartialEq, Eq)]
struct TrCall {
    line: usize,
    key: String,
}

fn main() -> ExitCode {
    let mut root = default_root();
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--root" => match args.next() {
                Some(value) => root = PathBuf::from(value),
                None => {
                    eprintln!("error: --root needs a path");
                    return ExitCode::from(2);
                }
            },
            "-h" | "--help" => {
                print!("{HELP}");
                return ExitCode::SUCCESS;
            }
            other => {
                eprintln!("error: unknown argument {other}");
                return ExitCode::from(2);
            }
        }
    }
    match run(&root) {
        Ok(drifted) => {
            if drifted {
                ExitCode::FAILURE
            } else {
                ExitCode::SUCCESS
            }
        }
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::from(2)
        }
    }
}

/// The crate lives at `<repo>/codex-rs/i18n-check`, so the repository root is
/// two levels up from the manifest directory.
fn default_root() -> PathBuf {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    manifest
        .parent()
        .and_then(Path::parent)
        .map_or_else(|| PathBuf::from("."), Path::to_path_buf)
}

fn run(root: &Path) -> Result<bool, String> {
    let dict_path = root.join("codex-rs/i18n/src/dict_zh.rs");
    let pairs = read_dictionary_pairs(&dict_path)?;
    let dictionary: BTreeSet<String> = pairs.iter().map(|(key, _)| key.clone()).collect();

    let source_root = root.join("codex-rs");
    let mut files = Vec::new();
    collect_rust_files(&source_root, &mut files)?;
    files.sort();

    let mut used: BTreeMap<String, Vec<String>> = BTreeMap::new();
    // Keys that reach `tr` through a variable rather than as a literal at the
    // call site. They count as used, but they are reported separately so that
    // the distinction is visible instead of silently folded into `used`.
    let mut bound: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut referencing_files = 0usize;
    for file in &files {
        let text = fs::read_to_string(file).map_err(|e| format!("{}: {e}", file.display()))?;
        if !file_contributes_keys(file, &text, &source_root) {
            continue;
        }
        // The checker's own source is a tool, not product copy: its literals are
        // syntax it parses, and counting them reports the parser as a rendered
        // key (it did -- `"text"` from the constant parser showed up as missing).
        if file.ends_with("i18n-check/src/main.rs")
            || file.ends_with("i18n-check/src/main_tests.rs")
        {
            continue;
        }
        referencing_files += 1;
        let location_base = file
            .strip_prefix(root)
            .map_or_else(|_| file.clone(), Path::to_path_buf);
        for call in extract_tr_calls(&text) {
            used.entry(call.key).or_default().push(format!(
                "{}:{}",
                location_base.display(),
                call.line
            ));
        }
        for call in extract_label_literals(&text)
            .into_iter()
            .chain(extract_const_literals(&text))
        {
            bound.entry(call.key).or_default().push(format!(
                "{}:{}",
                location_base.display(),
                call.line
            ));
        }
    }
    let bound_keys = bound.len();
    for (key, sites) in bound {
        used.entry(key).or_default().extend(sites);
    }

    let not_translated_path = root.join("codex-rs/i18n/not-translated.tsv");
    let not_translated = read_not_translated(&not_translated_path)?;
    // Entries that came from test fixtures. Registered instead of deleted: the
    // production/test split has been got wrong four times in this repo, and an
    // unregistered deletion of a real key is unrecoverable while an registered
    // exemption is visible in the report and reversible.
    let fixture_path = root.join("codex-rs/i18n/test-fixture-keys.tsv");
    let fixtures = read_not_translated(&fixture_path)?;
    let missing: Vec<(&String, &Vec<String>)> = used
        .iter()
        .filter(|(key, _)| !dictionary.contains(*key) && !not_translated.contains_key(*key))
        .collect();
    let unused: Vec<&String> = dictionary
        .iter()
        .filter(|key| !used.contains_key(*key) && !fixtures.contains_key(*key))
        .collect();
    let translated = used.keys().filter(|key| dictionary.contains(*key)).count();

    println!("i18n drift check");
    println!("  repository        : {}", root.display());
    println!(
        "  dictionary        : {} ({} entries)",
        dict_path.display(),
        dictionary.len()
    );
    println!(
        "  scanned           : {} rust files, {} referencing codex_i18n",
        files.len(),
        referencing_files
    );
    println!("  rendered keys     : {}", used.len());
    println!(
        "  bound keys        : {bound_keys} (rendered through a variable, e.g. `tr(current(), self.label)`)"
    );
    println!();

    println!(
        "[missing] rendered but not in the dictionary: {}",
        missing.len()
    );
    println!(
        "[missing] exempted, declared not-translatable (\u{2026}/i18n/not-translated.tsv): {}",
        not_translated.len()
    );
    println!(
        "[unused] exempted as test fixtures, not yet re-verified (\u{2026}/i18n/test-fixture-keys.tsv): {}",
        fixtures.len()
    );
    for (key, sites) in &missing {
        println!("  {key:?}");
        for site in sites.iter().take(3) {
            println!("      {site}");
        }
        if sites.len() > 3 {
            println!("      ... {} more", sites.len() - 3);
        }
    }
    println!();

    println!(
        "[unused] in the dictionary but rendered nowhere: {}",
        unused.len()
    );
    for key in &unused {
        println!("  {key:?}");
    }
    println!();

    let coverage = if used.is_empty() {
        100.0
    } else {
        translated as f64 * 100.0 / used.len() as f64
    };
    println!(
        "[coverage] translated {translated}/{} rendered keys ({coverage:.1}%)",
        used.len()
    );
    println!();

    // The spacing rule is a *style* rule, not a drift rule, but it is the same
    // kind of invariant: a convention that is only written down decays. Making
    // it fail here means the next batch either follows it or the build says why.
    let spacing = spacing_violations(&pairs);
    println!(
        "[spacing] CJK/Latin boundary spaces (i18n-glossary checklist 6): {}",
        spacing.len()
    );
    for (key, value) in &spacing {
        println!("  {value:?}");
        println!("      key {key:?}");
    }
    println!();

    let nested: Vec<(&PathBuf, usize)> = files
        .iter()
        .filter(|file| {
            fs::read_to_string(file)
                .map(|text| text.contains("codex_i18n"))
                .unwrap_or(false)
        })
        .flat_map(|file| {
            let text = fs::read_to_string(file).unwrap_or_default();
            nested_tr_calls(&text)
                .into_iter()
                .map(|line| (file, line))
                .collect::<Vec<_>>()
        })
        .collect();

    let duplicates = duplicate_keys(&pairs);
    println!(
        "[nested] nested tr/tr_with calls (double translation): {}",
        nested.len()
    );
    for (file, line) in &nested {
        println!("  {}:{}", file.display(), line);
    }
    println!();

    println!(
        "[duplicate] keys declared more than once (the last entry wins): {}",
        duplicates.len()
    );
    for (key, values) in &duplicates {
        println!("  {key:?}");
        for (index, value) in values.iter().enumerate() {
            let note = if index + 1 == values.len() {
                "effective"
            } else {
                "dead: the later entry overwrites it"
            };
            println!("      {value:?}  ({note})");
        }
    }
    println!();

    let placeholders = named_placeholder_hits(&pairs, &used);
    println!(
        "[placeholder] named placeholders (`substitute` only replaces positional `{{N}}`): {}",
        placeholders.len()
    );
    for (template, (names, sites)) in &placeholders {
        println!("  {template:?} carries {{{}}}", names.join("}, {"));
        for site in sites.iter().take(3) {
            println!("      {site}");
        }
        if sites.len() > 3 {
            println!("      ... {} more", sites.len() - 3);
        }
    }

    // Assets are data, so a translated copy is invisible to every check above.
    // Reconciling the two files is what makes a translation safe to add: the
    // rows are the identity, the template text is what may differ.
    let english_asset = root.join("codex-rs/core/assets/consequential_tool_message_templates.json");
    let translated_asset =
        root.join("codex-rs/core/assets/consequential_tool_message_templates.zh.json");
    println!();
    let asset_drift = if translated_asset.exists() {
        let drift = asset_row_drift(
            &asset_template_rows(&english_asset)?,
            &asset_template_rows(&translated_asset)?,
        );
        println!(
            "[asset] translated asset rows not mirrored: {}",
            drift.len()
        );
        for row in &drift {
            println!("  {row}");
        }
        drift
    } else {
        println!(
            "[asset] no translated asset yet ({})",
            translated_asset.display()
        );
        Vec::new()
    };

    Ok(!missing.is_empty()
        || !unused.is_empty()
        || !spacing.is_empty()
        || !duplicates.is_empty()
        || !placeholders.is_empty()
        || !asset_drift.is_empty()
        || !nested.is_empty())
}

/// Whether a source file can contribute keys the checker must reconcile.
///
/// The gate used to be "the text mentions `codex_i18n`", which silently skipped
/// any file that reaches `tr` through a glob import (`use super::*`): the crate
/// name never appears, so its calls were neither counted as rendered nor ever
/// reported `missing`. That hid 14 wrapped-but-untranslated keys in
/// `tui/src/chatwidget/review_popups.rs` while the gate stayed green (docs
/// §13.7). A file that *calls* `tr`/`tr_with` is in scope whether or not it
/// names the crate.
///
/// The crate that *defines* `tr` stays out: its own tests call `tr` with
/// fixture literals, and scanning those reported the fixtures as missing keys.
fn file_contributes_keys(file: &Path, text: &str, source_root: &Path) -> bool {
    if text.contains("codex_i18n") {
        return true;
    }
    if file.starts_with(source_root.join("i18n").join("src")) {
        return false;
    }
    !extract_tr_calls(text).is_empty()
}

fn collect_rust_files(dir: &Path, out: &mut Vec<PathBuf>) -> Result<(), String> {
    let entries = fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    for entry in entries {
        let entry = entry.map_err(|e| format!("{}: {e}", dir.display()))?;
        let path = entry.path();
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if path.is_dir() {
            if matches!(name.as_ref(), "target" | "vendor" | "node_modules") {
                continue;
            }
            collect_rust_files(&path, out)?;
        } else if name.ends_with(".rs") {
            out.push(path);
        }
    }
    Ok(())
}

/// Index of the `;` that closes a declaration starting at `from`.
///
/// The terminator has to be found by walking the source rather than by
/// searching for the first `;`: several English originals contain one (for
/// example `"Estimated current-thread credits (Enterprise workspaces only;
/// omitted when unavailable)"`), and treating that as the end of the array
/// silently drops every entry after it -- which is exactly the class of drift
/// this tool exists to catch, so it must not be introduced by the tool itself.
fn declaration_end(text: &str, from: usize) -> usize {
    // The array closes with `];` on a line of its own, so the body can be cut
    // there without a lexer. A `;` inside a key must never be the terminator:
    // several English originals contain one (e.g. "…workspaces only; omitted
    // when…"), and stopping there silently drops every entry after it -- the
    // very class of drift this tool exists to catch. The plain `;` search is
    // kept only as a fallback for one-line declarations such as the fixtures in
    // `main_tests.rs`.
    if let Some(offset) = text[from..].find("\n];") {
        return from + offset;
    }
    text[from..]
        .find(';')
        .map_or(text.len(), |offset| from + offset)
}

/// Whether `value` separates a CJK run from a Latin/digit run with a space.
///
/// `docs/plan/i18n-glossary.md` (checklist 6) requires mixed CJK/Latin text to
/// omit that space -- `启用{0}并记住此选择`, not `启用 {0} 并记住此选择` -- matching
/// qwen-code. Only *internal* boundaries count: a leading or trailing space is
/// layout, not mixed text. Several keys are positional (`" to move"` is appended
/// after a keybinding), so stripping those would change rendered output.
fn has_mixed_boundary_space(value: &str) -> bool {
    let chars: Vec<char> = value.chars().collect();
    if chars.len() < 3 {
        return false;
    }
    (1..chars.len() - 1).any(|i| {
        chars[i] == ' '
            && ((is_cjk(chars[i - 1]) && chars[i + 1].is_ascii_alphanumeric())
                || (chars[i - 1].is_ascii_alphanumeric() && is_cjk(chars[i + 1])))
    })
}

/// Whether `c` is a CJK ideograph (the ranges that carry Chinese text here).
fn is_cjk(c: char) -> bool {
    matches!(c, '\u{3400}'..='\u{4dbf}' | '\u{4e00}'..='\u{9fff}')
}

/// Dictionary entries whose translation breaks the no-space convention.
fn spacing_violations(pairs: &[(String, String)]) -> Vec<(&str, &str)> {
    pairs
        .iter()
        .filter(|(_, value)| has_mixed_boundary_space(value))
        .map(|(key, value)| (key.as_str(), value.as_str()))
        .collect()
}

/// The named placeholders in `template`, without duplicates.
///
/// A placeholder the engine cannot substitute: `substitute` replaces `{0}`,
/// `{1}`, ... and copies every other `{...}` through verbatim (the branch that
/// keeps a mistake visible rather than swallowing text). `{label}` therefore
/// reaches the screen as `{label}`, in the default language too.
fn named_placeholders(template: &str) -> Vec<String> {
    let mut found = Vec::new();
    let mut rest = template;
    while let Some(start) = rest.find('{') {
        let after = &rest[start + 1..];
        let Some(end) = after.find('}') else {
            break;
        };
        let token = &after[..end];
        // An index (`{0}`), an empty pair (`{}`) and prose (`{not a number}`)
        // are all left alone: only `{identifier}` is the shape the engine
        // cannot render.
        let named = token.starts_with(|c: char| c.is_ascii_alphabetic() || c == '_')
            && token.chars().all(|c| c.is_ascii_alphanumeric() || c == '_');
        if named && !found.iter().any(|existing| existing == token) {
            found.push(token.to_string());
        }
        rest = &after[end + 1..];
    }
    found
}

/// Records `template` in `hits` when it carries a named placeholder.
fn record_named_placeholders(
    hits: &mut BTreeMap<String, (Vec<String>, Vec<String>)>,
    template: &str,
    site: String,
) {
    let names = named_placeholders(template);
    if names.is_empty() {
        return;
    }
    let entry = hits
        .entry(template.to_string())
        .or_insert_with(|| (names, Vec::new()));
    if !entry.1.contains(&site) {
        entry.1.push(site);
    }
}

/// Every place a named placeholder survives to the screen, keyed by template.
///
/// Both sides are checked together, because either can carry the token: the
/// dictionary (key and translation) and the call site. None of the drift checks
/// above can see it -- the template still matches its key and its call site.
fn named_placeholder_hits(
    pairs: &[(String, String)],
    used: &BTreeMap<String, Vec<String>>,
) -> BTreeMap<String, (Vec<String>, Vec<String>)> {
    let mut hits: BTreeMap<String, (Vec<String>, Vec<String>)> = BTreeMap::new();
    for (key, value) in pairs {
        record_named_placeholders(&mut hits, key, "dictionary key".to_string());
        record_named_placeholders(&mut hits, value, "dictionary translation".to_string());
    }
    for (key, sites) in used {
        for site in sites {
            record_named_placeholders(&mut hits, key, site.clone());
        }
    }
    hits
}

/// Dictionary keys declared more than once, with every translation they were given.
///
/// `DICT_ZH` is `ENTRIES.iter().copied().collect()` -- a `HashMap`, so the last
/// entry for a key overwrites the earlier ones. Two batches can translate one
/// key differently and nothing above changes: the key stays unique in every
/// count, and the rendered text quietly follows whichever entry came last.
fn duplicate_keys(pairs: &[(String, String)]) -> Vec<(&str, Vec<&str>)> {
    let mut by_key: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for (key, value) in pairs {
        by_key.entry(key.as_str()).or_default().push(value.as_str());
    }
    by_key
        .into_iter()
        .filter(|(_, values)| values.len() > 1)
        .collect()
}

/// The `(connector_id, server_name, tool_title)` rows of an approval-template asset.
///
/// The asset is data, not source: `i18n-check` cannot see its strings, so a
/// translated copy is only safe if something compares the two files. That is
/// this function's job -- the row identity is the triple that selects a template
/// (`mcp_tool_approval_templates.rs`), not the template text, which is exactly
/// what a translation is allowed to change.
fn asset_template_rows(path: &Path) -> Result<Vec<String>, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let value: serde_json::Value =
        serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?;
    let rows = value
        .get("templates")
        .and_then(|templates| templates.as_array())
        .ok_or_else(|| format!("{}: no `templates` array", path.display()))?;
    let mut out = Vec::with_capacity(rows.len());
    for row in rows {
        let field = |name: &str| {
            row.get(name)
                .and_then(|value| value.as_str())
                .unwrap_or_default()
                .to_string()
        };
        out.push(format!(
            "{}|{}|{}",
            field("connector_id"),
            field("server_name"),
            field("tool_title")
        ));
    }
    Ok(out)
}

/// Rows present in one asset but not the other, in both directions.
fn asset_row_drift(english: &[String], translated: &[String]) -> Vec<String> {
    let english: BTreeSet<&String> = english.iter().collect();
    let translated: BTreeSet<&String> = translated.iter().collect();
    let mut drift: Vec<String> = english
        .difference(&translated)
        .map(|row| format!("missing from the translation: {row}"))
        .collect();
    drift.extend(
        translated
            .difference(&english)
            .map(|row| format!("not in the English asset: {row}")),
    );
    drift
}

/// Anchors that were judged **not translatable**, with the reason.
///
/// They are rendered (so they count as rendered keys) but translating them would
/// break behaviour -- the display value doubles as a comparison value, so the
/// answer submitted by the UI stops matching the English constant. Without this
/// list the gate reports them as `missing` forever, and "missing == 0" is the
/// core acceptance criterion, so the verdict has to be machine-readable rather
/// than living only in prose.
///
/// File format: one `key<TAB>file:line<TAB>reason` row per verdict, `#` comments.
/// The count is printed in the report so the exemption stays visible.
fn read_not_translated(path: &Path) -> Result<BTreeMap<String, String>, String> {
    if !path.exists() {
        return Ok(BTreeMap::new());
    }
    let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut out = BTreeMap::new();
    for line in text.lines() {
        let line = line.trim_end();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut fields = line.split('\t');
        let (Some(key), Some(site)) = (fields.next(), fields.next()) else {
            return Err(format!(
                "{}: row is not `key<TAB>file:line<TAB>reason`: {line:?}",
                path.display()
            ));
        };
        let reason = fields.next().unwrap_or_default();
        if reason.trim().is_empty() {
            return Err(format!(
                "{}: row for {key:?} has no reason; every exemption must say why",
                path.display()
            ));
        }
        out.insert(key.to_string(), site.to_string());
    }
    Ok(out)
}

/// Nested `tr`/`tr_with` calls, which double-translate.
///
/// `tr(current(), tr(current(), "…"))` looks harmless and renders correctly in
/// English, but the inner call already returns the translation, and the outer
/// one then looks that up again -- a no-op in English and a missed lookup in
/// Chinese. Round 33 fixed one of these by hand; this makes the property
/// checkable instead of relying on somebody grepping for it.
///
/// Detection is deliberately lexical and narrow: an identifier `tr`/`tr_with`
/// immediately followed by `(` whose first argument itself starts with a
/// `tr`/`tr_with` call.
///
/// Returns **line numbers** (1-based), not byte offsets.
fn nested_tr_calls(text: &str) -> Vec<usize> {
    let mut lines = Vec::new();
    let bytes = text.as_bytes();
    let mut i = 0usize;
    while i < bytes.len() {
        if let Some(next) = skip_ignorable(text, i) {
            i = next;
            continue;
        }
        if is_identifier_start(bytes[i]) {
            let start = i;
            while i < bytes.len() && is_identifier_continue(bytes[i]) {
                i += 1;
            }
            let ident = &text[start..i];
            if (ident == "tr" || ident == "tr_with") && is_followed_by_nested_tr(text, i) {
                lines.push(line_number(text, start));
            }
            continue;
        }
        i += 1;
    }
    lines
}

/// Whether the call whose `(` follows `after_ident` passes a `tr` call as its
/// first argument, e.g. `(current(), tr(...))`.
fn is_followed_by_nested_tr(text: &str, after_ident: usize) -> bool {
    let open = skip_trivia(text, after_ident);
    if text.as_bytes().get(open) != Some(&b'(') {
        return false;
    }
    let mut at = skip_trivia(text, open + 1);
    // Walk the outer argument list looking for an inner call at argument start.
    loop {
        let bytes = text.as_bytes();
        if !bytes.get(at).copied().is_some_and(is_identifier_start) {
            return false;
        }
        let start = at;
        while at < bytes.len() && is_identifier_continue(bytes[at]) {
            at += 1;
        }
        if &text[start..at] == "tr" || &text[start..at] == "tr_with" {
            return true;
        }
        let after = argument_end(text, start);
        match after {
            Some(end) if bytes.get(end) == Some(&b',') => at = skip_trivia(text, end + 1),
            _ => return false,
        }
    }
}

/// Reads the `(english, translation)` pairs of `static ENTRIES`.
fn read_dictionary_pairs(path: &Path) -> Result<Vec<(String, String)>, String> {
    let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let declaration = text
        .find("static ENTRIES")
        .ok_or_else(|| format!("{}: no `static ENTRIES` declaration", path.display()))?;
    let end = declaration_end(&text, declaration);
    let body = &text[declaration..end];

    let mut pairs = Vec::new();
    let mut i = 0usize;
    while i < body.len() {
        // Comments inside the array can contain quoted text ("…", "…"), and a
        // `read_string` scan would happily read those as entries -- which shows
        // up as a phantom "unused" key. Skip *comments only* here: skipping
        // "ignorable" text would skip the string literals themselves, which is
        // how the first attempt at this fix parsed zero entries.
        if let Some(next) = skip_comment(body, i) {
            i = next;
            continue;
        }
        let Some((key, after_key)) = read_string(body, i) else {
            i += 1;
            continue;
        };
        let comma = skip_trivia(body, after_key);
        if body.as_bytes().get(comma) != Some(&b',') {
            i = after_key;
            continue;
        }
        let value_start = skip_trivia(body, comma + 1);
        match read_string(body, value_start) {
            Some((value, after_value)) => {
                pairs.push((key, value));
                i = after_value;
            }
            None => i = after_key,
        }
    }
    Ok(pairs)
}

/// Every `tr(<key>)` call whose first argument is a string literal.
fn extract_tr_calls(text: &str) -> Vec<TrCall> {
    let bytes = text.as_bytes();
    let mut calls = Vec::new();
    let mut i = 0usize;
    while i < bytes.len() {
        if let Some(next) = skip_ignorable(text, i) {
            i = next;
            continue;
        }
        if is_identifier_start(bytes[i]) {
            let start = i;
            while i < bytes.len() && is_identifier_continue(bytes[i]) {
                i += 1;
            }
            let ident = &text[start..i];
            // `tr_with` is the interpolation form. It has the same
            // `(lang, key, args)` shape, so the same "first literal argument"
            // rule applies -- and it has to be seen, because a template that is
            // rendered on every keystroke is not an unused key.
            if (ident == "tr" || ident == "tr_with")
                && let Some(key) = parse_tr_call(text, i)
            {
                calls.push(TrCall {
                    line: line_number(text, start),
                    key,
                });
            }
            continue;
        }
        i += 1;
    }
    calls
}

/// Parses the key argument of the call whose `(` follows `after_ident`.
///
/// The signature is `tr(lang, key)`, so the key is the second positional
/// argument; taking the first *string literal* argument rather than a fixed
/// position keeps working for `tr(Lang::En, "…")` too.
fn parse_tr_call(text: &str, after_ident: usize) -> Option<String> {
    let open = skip_trivia(text, after_ident);
    if text.as_bytes().get(open) != Some(&b'(') {
        return None;
    }
    let mut start = open + 1;
    loop {
        let end = argument_end(text, start)?;
        let mut argument = text[start..end].trim();
        if let Some(rest) = argument.strip_prefix('&') {
            argument = rest.trim_start();
        }
        if let Some((value, _)) = read_string(argument, /*i*/ 0) {
            return Some(value);
        }
        if text.as_bytes().get(end) != Some(&b',') {
            return None;
        }
        start = end + 1;
    }
}

/// Field names whose string literals are rendered through a variable.
///
/// A table cannot always call `tr` where the text is written -- a `const` cannot
/// call a non-`const` function -- so some tables keep their English text in a
/// field and render it at the use site as `tr(current(), self.label)`. The
/// literal then never appears at a call site, and every such key would be
/// reported as unused, which is the report that invites deleting a translation
/// that is on screen constantly.
///
/// The list is deliberately just `label`: it is the one field name the codebase
/// actually renders that way. Widening it to the obvious candidates (`name`,
/// `description`, `*_name`, `*_description`) was measured and rejected -- it
/// immediately turned 33 not-yet-wired strings into "missing", i.e. it made the
/// report noisy about work that is simply not done yet. The structural fix for
/// such a table is to turn it into a function instead (see
/// `plugin_catalog::remote_marketplace_sections`), which puts the keys back at
/// real call sites and keeps this rule narrow.
fn is_bound_key_field(name: &str) -> bool {
    name == "label"
}

/// String constants that are rendered somewhere: `const NAME: &str = "text"`.
///
/// The third shape an English original can take. A `const` cannot call `tr`, so
/// tables keep the text in a constant and splice it into a rendered string
/// (`format!("{prefix}{NAME}")`, `name:`/`Line`/`Span`). The literal is not at a
/// `tr` call site and not in a `label:` field, so without this pass the key looks
/// unused -- and "unused" is the report that invites deleting a translation that
/// is on screen. That is exactly what happened to `None of the above`
/// (`request_user_input/mod.rs:64` -> `:477` `name: format!("{prefix_label}{OTHER_OPTION_LABEL}")`).
///
/// A constant that is never rendered (`MIN_HEIGHT`, a telemetry name) does not
/// count: it has to appear in a rendering expression.
fn extract_const_literals(text: &str) -> Vec<TrCall> {
    let mut calls = Vec::new();
    let mut i = 0usize;
    while i < text.len() {
        let Some(offset) = text[i..].find("const ") else {
            break;
        };
        let start = i + offset;
        i = start + "const ".len();
        let Some((name, after_name)) = read_identifier(text, i) else {
            continue;
        };
        let Some((value, _after_value)) = parse_const_string_value(text, after_name) else {
            continue;
        };
        // Three shapes are structure, not copy: empty strings, whitespace-only
        // strings, and escape sequences (terminal control, box drawing). They are
        // rendered, but nothing renders them *as text to read*.
        let is_structure = value.trim().is_empty()
            || value.contains('\u{1b}')
            || value.chars().all(|c| !c.is_alphanumeric());
        if is_structure || !const_is_rendered(text, &name, start) {
            continue;
        }
        calls.push(TrCall {
            line: line_number(text, start),
            key: value,
        });
    }
    calls
}

/// Reads an identifier at `i`, returning it and the index just past it.
fn read_identifier(text: &str, i: usize) -> Option<(String, usize)> {
    let bytes = text.as_bytes();
    if !bytes.get(i).copied().is_some_and(is_identifier_start) {
        return None;
    }
    let mut end = i;
    while bytes.get(end).copied().is_some_and(is_identifier_continue) {
        end += 1;
    }
    Some((text[i..end].to_string(), end))
}

/// Parses `: &str = "<literal>";` after a constant's name.
fn parse_const_string_value(text: &str, after_name: usize) -> Option<(String, usize)> {
    let colon = skip_trivia(text, after_name);
    if text.as_bytes().get(colon) != Some(&b':') {
        return None;
    }
    let mut cursor = skip_trivia(text, colon + 1);
    if !text[cursor..].starts_with("&str") && !text[cursor..].starts_with("&'static str") {
        return None;
    }
    cursor = skip_trivia(text, text[cursor..].find("str")? + cursor + 3);
    if text.as_bytes().get(cursor) != Some(&b'=') {
        return None;
    }
    let value = skip_trivia(text, cursor + 1);
    read_string(text, value)
}

/// Whether `name` is spliced into a *text slot* that reaches the screen.
///
/// Deliberately narrow. `format!` / `push_str` also assemble prompts for the
/// model, telemetry names and config keys -- `"Read the Codex goal objective
/// file at "`, `".config.toml"`, `"features.multi_agent_v2.tool_namespace"` --
/// and those stay English by design (i18n-design §3.6). A constant whose text a
/// user reads does so through a slot that names it as text.
fn const_is_rendered(text: &str, name: &str, definition: usize) -> bool {
    const RENDER_SLOTS: [&str; 6] = [
        "name:",
        "title:",
        "label:",
        "description:",
        "Line::from",
        "Span::from",
    ];
    let mut from = definition;
    while let Some(offset) = text[from..].find(name) {
        let at = from + offset;
        from = at + name.len();
        // Skip the definition itself.
        if at < definition + name.len() + "const ".len() {
            continue;
        }
        let line_start = text[..at].rfind('\n').map_or(0, |nl| nl + 1);
        let line = &text[line_start..text[at..].find('\n').map_or(text.len(), |nl| at + nl)];
        if RENDER_SLOTS.iter().any(|slot| line.contains(slot)) {
            return true;
        }
    }
    false
}

/// Every `label: "<literal>"` (and the other bound-key fields) in the source.
///
/// Shortcut descriptors and `const` tables keep their English text in a field
/// and render it through `tr(current(), self.field)` where it is used, so the
/// key never shows up as a literal inside a `tr(..)` call. Without this pass
/// those keys look unused. Only string literals count, so `label: field` and
/// `name: some_expr` are ignored.
///
/// Test code is skipped: a fixture's `label: "Calendar"` is a value to compare
/// against, not a key anything renders through `tr`. Scanning it turned a
/// fixture into a phantom "missing" key the moment a previously i18n-free file
/// gained its first `use codex_i18n::..` (the file-level gate below).
fn extract_label_literals(text: &str) -> Vec<TrCall> {
    extract_label_literals_in(text, /*skip_tests*/ true)
}

/// [`extract_label_literals`] with the test-code filter switchable, so the rule
/// itself is testable without a whole file.
fn extract_label_literals_in(text: &str, skip_tests: bool) -> Vec<TrCall> {
    let production_end = if skip_tests {
        test_module_start(text).unwrap_or(text.len())
    } else {
        text.len()
    };
    let text = &text[..production_end];
    let bytes = text.as_bytes();
    let mut calls = Vec::new();
    let mut i = 0usize;
    while i < bytes.len() {
        if let Some(next) = skip_ignorable(text, i) {
            i = next;
            continue;
        }
        if is_identifier_start(bytes[i]) {
            let start = i;
            while i < bytes.len() && is_identifier_continue(bytes[i]) {
                i += 1;
            }
            let ident = &text[start..i];
            if is_bound_key_field(ident)
                && let Some(key) = parse_label_literal(text, i)
            {
                // An empty label is a layout placeholder, not text, so it is
                // not a key anybody could translate -- reporting it as
                // "rendered without a translation" would be noise that
                // trains the reader to ignore the report.
                if !key.is_empty() {
                    calls.push(TrCall {
                        line: line_number(text, start),
                        key,
                    });
                }
            }
            continue;
        }
        i += 1;
    }
    calls
}

/// Offset of the first `#[cfg(test)]`, i.e. where production code ends.
///
/// Deliberately a plain search rather than a parser: this tool is lexical
/// everywhere else, and every i18n-relevant `#[cfg(test)]` in this repo is
/// spelled exactly that way.
fn test_module_start(text: &str) -> Option<usize> {
    text.find("#[cfg(test)]")
}

/// Parses the literal in `label: "<literal>"`, where `label` ends at
/// `after_ident`.
fn parse_label_literal(text: &str, after_ident: usize) -> Option<String> {
    let colon = skip_trivia(text, after_ident);
    if text.as_bytes().get(colon) != Some(&b':') {
        return None;
    }
    let value = skip_trivia(text, colon + 1);
    read_string(text, value).map(|(value, _)| value)
}

/// Index of the top-level `,` or closing `)` that ends the argument at `start`.
fn argument_end(text: &str, start: usize) -> Option<usize> {
    let bytes = text.as_bytes();
    let mut depth = 0usize;
    let mut i = start;
    while i < bytes.len() {
        if let Some(next) = skip_ignorable(text, i) {
            i = next;
            continue;
        }
        match bytes[i] {
            b'(' | b'[' | b'{' => depth += 1,
            b')' | b']' | b'}' => {
                if depth == 0 {
                    return Some(i);
                }
                depth -= 1;
            }
            b',' if depth == 0 => return Some(i),
            _ => {}
        }
        i += 1;
    }
    None
}

/// Skips comments, string literals and char literals starting at `i`.
fn skip_ignorable(text: &str, i: usize) -> Option<usize> {
    let bytes = text.as_bytes();
    if let Some(next) = skip_comment(text, i) {
        return Some(next);
    }
    if let Some((_, after)) = read_string(text, i) {
        return Some(after);
    }
    // Char literal (`'x'`, `'\n'`, `'\u{1F600}'`), but not a lifetime.
    if bytes.get(i) == Some(&b'\'')
        && let Some(after) = read_char_literal(text, i)
    {
        return Some(after);
    }
    None
}

/// Skips a `//` or (nested) `/* */` comment starting at `i`.
fn skip_comment(text: &str, i: usize) -> Option<usize> {
    let bytes = text.as_bytes();
    if bytes.get(i) == Some(&b'/') && bytes.get(i + 1) == Some(&b'/') {
        return Some(text[i..].find('\n').map_or(text.len(), |offset| i + offset));
    }
    if bytes.get(i) == Some(&b'/') && bytes.get(i + 1) == Some(&b'*') {
        let mut depth = 1usize;
        let mut j = i + 2;
        while j < bytes.len() && depth > 0 {
            if bytes[j..].starts_with(b"/*") {
                depth += 1;
                j += 2;
            } else if bytes[j..].starts_with(b"*/") {
                depth -= 1;
                j += 2;
            } else {
                j += 1;
            }
        }
        return Some(j);
    }
    None
}

fn read_char_literal(text: &str, i: usize) -> Option<usize> {
    let rest = text.get(i + 1..)?;
    let mut chars = rest.char_indices();
    let (_, first) = chars.next()?;
    if first == '\\' {
        // Skip the escape: `\n`, `\x41`, `\u{1F600}`.
        let escape_rest = &rest[1..];
        let next = escape_rest.chars().next()?;
        let consumed = if next == 'u' || next == 'x' {
            let close = escape_rest
                .find('}')
                .or_else(|| escape_rest.char_indices().nth(2).map(|(i, _)| i))?;
            (1 + close + 1).min(escape_rest.len())
        } else {
            next.len_utf8()
        };
        let after_escape = i + 1 + 1 + consumed;
        return (text.as_bytes().get(after_escape) == Some(&b'\'')).then_some(after_escape + 1);
    }
    let after_char = i + 1 + first.len_utf8();
    (text.as_bytes().get(after_char) == Some(&b'\'')).then_some(after_char + 1)
}

fn skip_trivia(text: &str, i: usize) -> usize {
    let bytes = text.as_bytes();
    let mut j = i;
    loop {
        while j < bytes.len() && bytes[j].is_ascii_whitespace() {
            j += 1;
        }
        // Only whitespace and comments: skipping string literals here would
        // step over the very argument being read.
        match skip_comment(text, j) {
            Some(next) if next > j => j = next,
            _ => return j,
        }
    }
}

/// Decodes a Rust string literal starting at `i`, returning its value and the
/// index just past the literal.
fn read_string(text: &str, i: usize) -> Option<(String, usize)> {
    let bytes = text.as_bytes();
    let mut cursor = i;
    if matches!(bytes.get(cursor), Some(b'b' | b'c')) {
        cursor += 1;
    }
    if bytes.get(cursor) == Some(&b'r') {
        let mut hashes = 0usize;
        let mut j = cursor + 1;
        while bytes.get(j) == Some(&b'#') {
            hashes += 1;
            j += 1;
        }
        if bytes.get(j) != Some(&b'"') {
            return None;
        }
        let body_start = j + 1;
        let closer = format!("\"{}", "#".repeat(hashes));
        let end = text.get(body_start..)?.find(&closer)? + body_start;
        return Some((text[body_start..end].to_string(), end + closer.len()));
    }
    if bytes.get(cursor) != Some(&b'"') {
        return None;
    }
    let mut value = String::new();
    let mut j = cursor + 1;
    while j < bytes.len() {
        match bytes[j] {
            b'"' => return Some((value, j + 1)),
            b'\\' => {
                let (decoded, after) = read_escape(text, j)?;
                value.push(decoded);
                j = after;
            }
            _ => {
                let ch = text[j..].chars().next()?;
                value.push(ch);
                j += ch.len_utf8();
            }
        }
    }
    None
}

fn read_escape(text: &str, backslash: usize) -> Option<(char, usize)> {
    let rest = text.get(backslash + 1..)?;
    let escaped = rest.chars().next()?;
    let simple = match escaped {
        'n' => '\n',
        'r' => '\r',
        't' => '\t',
        '0' => '\0',
        '\\' => '\\',
        '\'' => '\'',
        '"' => '"',
        'x' => {
            let digits = rest.get(1..3)?;
            let code = u8::from_str_radix(digits, 16).ok()?;
            return Some((char::from(code), backslash + 1 + 3));
        }
        'u' => {
            let open = rest.find('{')?;
            let close = rest.find('}')?;
            let digits = rest.get(open + 1..close)?;
            let code = u32::from_str_radix(&digits.replace('_', ""), 16).ok()?;
            return Some((char::from_u32(code)?, backslash + 1 + close + 1));
        }
        other => other,
    };
    Some((simple, backslash + 1 + escaped.len_utf8()))
}

fn line_number(text: &str, index: usize) -> usize {
    text.as_bytes()[..index.min(text.len())]
        .iter()
        .filter(|byte| **byte == b'\n')
        .count()
        + 1
}

fn is_identifier_start(byte: u8) -> bool {
    byte.is_ascii_alphabetic() || byte == b'_'
}

fn is_identifier_continue(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

#[cfg(test)]
#[path = "main_tests.rs"]
mod tests;
