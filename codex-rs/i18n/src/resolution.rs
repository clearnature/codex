//! Where the UI language comes from.
//!
//! `docs/plan/i18n-design.md` §4 left the locale entry point as an open
//! decision with four candidates. This is candidate ④, the combination,
//! because each of the other three leaves a documented case unserved: a
//! configuration-only entry cannot be overridden for a single invocation, a
//! flag-only entry cannot be set once for a whole machine, an environment-only
//! entry cannot be corrected in a shell that exports a stale value, and none
//! of the three picks up the operating system's own setting.
//!
//! Precedence, highest first:
//!
//! 1. `--lang` on the command line
//! 2. `locale` in `config.toml`
//! 3. the environment: `LC_ALL`, then `LANG`
//! 4. the operating system's own setting
//!
//! Precedence is decided by *presence*, never by whether a value names a
//! language that has translations. `--lang=en` therefore beats a configuration
//! that says `zh` -- an explicit request is not a hint -- and `--lang=fr` beats
//! `zh` too, then degrades to English, because [`parse_lang`] maps everything it
//! does not recognise to [`Lang::En`]. An unknown language is never an error and
//! never produces a half-translated interface.
//!
//! This module is the only part of the crate that reads the environment or
//! asks the operating system anything, and it does so through two narrow seams
//! -- [`Env`] and [`resolve`] -- so that the precedence rule itself stays a pure
//! function which tests can drive over every combination, and so that the
//! rendering path (`crate::tr`) keeps being a pure lookup.

use crate::lang::Lang;
use crate::lang::parse_lang;

/// The locale-bearing environment variables, captured as plain data.
///
/// Callers hand over what the process actually has instead of letting this
/// module read the environment itself, which is what keeps [`resolve`] pure and
/// its tests independent of the machine they run on.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Env {
    /// `LC_ALL`. Outranks `LANG` whenever it is set to a non-empty value, which
    /// is what POSIX specifies.
    pub lc_all: Option<String>,
    /// `LANG`.
    pub lang: Option<String>,
}

impl Env {
    /// Reads `LC_ALL` and `LANG` from the running process.
    ///
    /// Empty values become `None` here rather than at the point of use, so that
    /// "set but empty" -- which a shell allows and which POSIX treats as unset
    /// -- never masks a lower-precedence source.
    #[must_use]
    pub fn from_process() -> Self {
        Self {
            lc_all: non_empty(std::env::var("LC_ALL").ok()),
            lang: non_empty(std::env::var("LANG").ok()),
        }
    }

    /// The first variable that actually carries a value.
    ///
    /// A variable that is present but empty -- or all whitespace -- does not
    /// carry one, so it is skipped here just as [`Env::from_process`] skips it.
    #[must_use]
    pub fn first(&self) -> Option<&str> {
        [self.lc_all.as_deref(), self.lang.as_deref()]
            .into_iter()
            .flatten()
            .find(|value| !value.trim().is_empty())
    }
}

/// Drops absent, empty and whitespace-only values.
fn non_empty(raw: Option<String>) -> Option<String> {
    raw.filter(|value| !value.trim().is_empty())
}

/// Resolves the UI language from every source, in precedence order.
///
/// `system_lang` is the operating system's own locale, probed by
/// [`system_locale`]; it is passed in rather than read here so that this
/// function stays pure.
#[must_use]
pub fn resolve(
    cli_lang: Option<&str>,
    config_lang: Option<&str>,
    env: &Env,
    system_lang: Option<&str>,
) -> Lang {
    [cli_lang, config_lang, env.first(), system_lang]
        .into_iter()
        .flatten()
        .find(|raw| !raw.trim().is_empty())
        .map_or(Lang::En, parse_lang)
}

/// Resolves the UI language the way a front end does at startup.
///
/// This is the one-call form of [`resolve`]: it captures the process
/// environment and probes the operating system, then applies the same
/// precedence. Front ends call it once and publish the answer with
/// [`crate::set_current`].
#[must_use]
pub fn resolve_from_process(cli_lang: Option<&str>, config_lang: Option<&str>) -> Lang {
    let system_lang = system_locale();
    resolve(
        cli_lang,
        config_lang,
        &Env::from_process(),
        system_lang.as_deref(),
    )
}

/// The operating system's locale, if it has one.
///
/// Two spellings matter in practice and both are what `sys-locale` returns: a
/// real tag such as `zh_CN.UTF-8` or `en_US.UTF-8`, and `C` / `POSIX`, which is
/// what a process with no locale configured reports. Both go through
/// [`parse_lang`], so `C` lands on English and `zh_CN.UTF-8` lands on Chinese.
#[must_use]
pub fn system_locale() -> Option<String> {
    sys_locale::get_locale()
}

#[cfg(test)]
#[path = "resolution_tests.rs"]
mod tests;
