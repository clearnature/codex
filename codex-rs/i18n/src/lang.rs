/// UI language.
///
/// This is the internal identifier. It is intentionally not the same thing as
/// the locale tag a user writes in `LANG` or in configuration -- [`parse_lang`]
/// maps the latter onto it.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Lang {
    /// English. The default, and the fallback for every untranslated string.
    #[default]
    En,
    /// Simplified Chinese.
    Zh,
}

/// Resolves a locale tag to a [`Lang`].
///
/// Accepts the spellings users actually end up with in `LANG`, in configuration
/// or on a command line: `zh`, `zh-CN`, `zh-hans`, `中文`, `cn`. Matching is
/// case-insensitive, `_` is treated as `-`, and the POSIX codeset and modifier
/// suffixes are ignored, so `ZH_cn`, `zh_CN.UTF-8` and `zh-CN@euro` all resolve
/// the same way as `zh-CN`. The suffixes have to be stripped here rather than by
/// callers: `sys-locale` reports what the operating system has, which on Linux
/// is routinely `zh_CN.UTF-8`, and a language choice that depends on the
/// codeset is not a language choice.
///
/// Anything unrecognized resolves to [`Lang::En`]. An unknown locale is never an
/// error, and never yields a partially translated UI.
#[must_use]
pub fn parse_lang(raw: &str) -> Lang {
    let normalized = raw.trim().to_ascii_lowercase().replace('_', "-");
    let language = normalized.split(['.', '@']).next().unwrap_or(&normalized);
    match language {
        "zh" | "cn" | "zh-cn" | "zh-hans" | "中文" => Lang::Zh,
        _ => Lang::En,
    }
}

#[cfg(test)]
#[path = "lang_tests.rs"]
mod tests;
