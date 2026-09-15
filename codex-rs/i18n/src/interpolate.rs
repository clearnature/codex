//! Interpolation for strings that carry runtime values.
//!
//! A large share of the user-facing text is not a constant: `format!` templates
//! like `"Deleted {count} files"` are the norm in the CLI/TUI, and the locale
//! chain has to keep working for them. The design
//! (`docs/plan/i18n-design.md` §6) asks for the same answer `qwen-code` uses:
//! the English template *is* the key, and the translation carries the same
//! placeholders, which are substituted after translation rather than before.
//!
//! Placeholders are positional: `{0}`, `{1}`, ... refer to `args[0]`, `args[1]`,
//! .... Rust's own `format!` cannot be used here because its format string must
//! be a literal known at compile time, and the whole point is to format a string
//! that is only known after the dictionary lookup.
//!
//! Two properties this module deliberately guarantees:
//!
//! * **English is unchanged.** `tr_with(Lang::En, key, args)` substitutes into
//!   `key` itself, so a call site that previously built a `format!` string keeps
//!   producing the same text whenever English is selected -- the same property
//!   [`crate::tr`] gives constant strings.
//! * **A bad translation never panics.** An index with no matching argument is
//!   left in the output verbatim, and an argument that no placeholder consumes is
//!   simply unused. A translator who reorders, drops or invents a placeholder
//!   produces wrong-looking text at worst, never a crash inside rendering --
//!   `interpolate_tests` pins that down, and the dictionary tests assert that the
//!   entries shipped today keep their placeholders.

use crate::lang::Lang;
use crate::tr;

/// Renders `key` in `lang`, substituting `{0}`, `{1}`, ... with `args`.
///
/// Returns an owned `String` because the result may be a translation that is
/// only known at runtime.
#[must_use]
pub fn tr_with(lang: Lang, key: &'static str, args: &[&str]) -> String {
    substitute(tr(lang, key), args)
}

/// Replaces `{N}` tokens in `template` with `args[N]`.
///
/// Exposed separately from [`tr_with`] so that the substitution rule can be
/// tested without a dictionary entry, and so that call sites holding an already
/// translated string (or an English original that is not `'static`) can use it.
#[must_use]
pub fn substitute(template: &str, args: &[&str]) -> String {
    let mut out = String::with_capacity(template.len());
    let mut rest = template;
    while let Some(start) = rest.find('{') {
        out.push_str(&rest[..start]);
        let after_brace = &rest[start + 1..];
        let Some(end) = after_brace.find('}') else {
            // An unclosed brace is not a placeholder; copy it through.
            out.push_str(&rest[start..]);
            return out;
        };
        let token = &after_brace[..end];
        match token
            .parse::<usize>()
            .ok()
            .and_then(|index| args.get(index))
        {
            Some(value) => out.push_str(value),
            // Unknown index: keep the token as written rather than dropping it,
            // so the mistake is visible instead of silently swallowing text.
            None => out.push_str(&rest[start..start + end + 2]),
        }
        rest = &after_brace[end + 1..];
    }
    out.push_str(rest);
    out
}

/// The placeholder indices a template uses, in the order they appear, with
/// duplicates removed.
///
/// This is the parity check a translation has to pass: whatever `{N}` the
/// English template uses, the translation must use the same set, or the
/// translated text is missing a value it was written to show.
#[must_use]
pub fn placeholders(template: &str) -> Vec<usize> {
    let mut found: Vec<usize> = Vec::new();
    let mut rest = template;
    while let Some(start) = rest.find('{') {
        let after_brace = &rest[start + 1..];
        let Some(end) = after_brace.find('}') else {
            break;
        };
        if let Ok(index) = after_brace[..end].parse::<usize>() {
            if !found.contains(&index) {
                found.push(index);
            }
        }
        rest = &after_brace[end + 1..];
    }
    found
}

#[cfg(test)]
#[path = "interpolate_tests.rs"]
mod tests;
