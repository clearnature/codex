//! Localization for Codex's user-facing strings.
//!
//! The translation key *is* the English source text. Rendering looks the key up
//! through [`tr`], so the English output stays byte-identical to the pre-i18n
//! shell and any missing translation degrades gracefully to English instead of
//! showing a placeholder or an empty string.
//!
//! The crate has no configuration of its own, no workspace-internal
//! dependency and no I/O on the rendering path. That keeps it usable from
//! `codex-tui`, `codex-cli` and `codex-exec` without touching the existing
//! dependency boundaries -- notably `tui` does not depend on `core`, so the
//! shared layer cannot live there.
//!
//! Two things do look outside the process, and both are deliberately confined
//! to [`resolution`], whose precedence rule is a pure function: reading the
//! locale environment variables and asking the operating system for its
//! locale. Everything a rendering site touches -- [`tr`], [`current`] -- is a
//! pure lookup over data that a front end published once at startup.
//!
//! Translating strings is out of scope for this stage; see
//! `docs/plan/i18n-verification.md` for the current hypothesis under test.

mod current;
mod dict_zh;
mod interpolate;
mod lang;
mod resolution;

pub use current::current;
pub use current::set_current;
pub use interpolate::placeholders;
pub use interpolate::substitute;
pub use interpolate::tr_with;
pub use lang::Lang;
pub use lang::parse_lang;
pub use resolution::Env;
pub use resolution::resolve;
pub use resolution::resolve_from_process;
pub use resolution::system_locale;

/// Renders `key` in `lang`.
///
/// `key` is the English source text itself. [`Lang::En`] returns it unchanged,
/// which is what keeps the existing snapshots stable; [`Lang::Zh`] falls back
/// to the English key whenever the dictionary has no entry for it.
#[must_use]
pub fn tr(lang: Lang, key: &'static str) -> &'static str {
    match lang {
        Lang::En => key,
        Lang::Zh => dict_zh::DICT_ZH.get(key).copied().unwrap_or(key),
    }
}

#[cfg(test)]
#[path = "lib_tests.rs"]
mod tests;
