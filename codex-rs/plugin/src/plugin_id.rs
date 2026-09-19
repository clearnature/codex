//! Stable plugin identifier parsing and validation shared with the plugin cache.

use codex_i18n::current;
use codex_i18n::tr;
use codex_i18n::tr_with;
#[derive(Debug, thiserror::Error)]
pub enum PluginIdError {
    #[error("{0}")]
    Invalid(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PluginId {
    pub plugin_name: String,
    pub marketplace_name: String,
}

impl PluginId {
    pub fn new(plugin_name: String, marketplace_name: String) -> Result<Self, PluginIdError> {
        validate_plugin_segment(&plugin_name, "plugin name").map_err(PluginIdError::Invalid)?;
        validate_plugin_segment(&marketplace_name, "marketplace name")
            .map_err(PluginIdError::Invalid)?;
        Ok(Self {
            plugin_name,
            marketplace_name,
        })
    }

    pub fn parse(plugin_key: &str) -> Result<Self, PluginIdError> {
        let Some((plugin_name, marketplace_name)) = plugin_key.rsplit_once('@') else {
            return Err(PluginIdError::Invalid(tr_with(
                current(),
                "invalid plugin key `{0}`; expected <plugin>@<marketplace>",
                &[plugin_key],
            )));
        };
        if plugin_name.is_empty() || marketplace_name.is_empty() {
            return Err(PluginIdError::Invalid(tr_with(
                current(),
                "invalid plugin key `{0}`; expected <plugin>@<marketplace>",
                &[plugin_key],
            )));
        }

        Self::new(plugin_name.to_string(), marketplace_name.to_string()).map_err(|err| match err {
            PluginIdError::Invalid(message) => {
                PluginIdError::Invalid(tr_with(current(), "{0} in `{1}`", &[&message, plugin_key]))
            }
        })
    }

    pub fn as_key(&self) -> String {
        format!("{}@{}", self.plugin_name, self.marketplace_name)
    }
}

/// Validates a single path segment used in plugin IDs and cache layout.
pub fn validate_plugin_segment(segment: &str, kind: &str) -> Result<(), String> {
    // `kind` 是控制流键（下面的比较用它），渲染用的标签在这里本地化：
    let kind_label = match kind {
        "plugin name" => tr(current(), "plugin name"),
        "marketplace name" => tr(current(), "marketplace name"),
        other => other,
    };
    if segment.is_empty() {
        return Err(tr_with(
            current(),
            "invalid {0}: must not be empty",
            &[kind_label],
        ));
    }
    let allow_dots = kind == "plugin name";
    if allow_dots && matches!(segment, "." | "..") {
        return Err(tr_with(
            current(),
            "invalid {0}: path traversal is not allowed",
            &[kind_label],
        ));
    }
    if allow_dots && (segment.starts_with('.') || segment.ends_with('.') || segment.contains(".."))
    {
        return Err(tr_with(
            current(),
            "invalid {0}: dots must separate non-empty name segments",
            &[kind_label],
        ));
    }
    if !segment
        .chars()
        .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_') || allow_dots && ch == '.')
    {
        let allowed_characters = if allow_dots {
            tr(current(), "ASCII letters, digits, `.`, `_`, and `-`")
        } else {
            tr(current(), "ASCII letters, digits, `_`, and `-`")
        };
        return Err(tr_with(
            current(),
            "invalid {0}: only {1} are allowed",
            &[kind_label, allowed_characters],
        ));
    }
    Ok(())
}

#[cfg(test)]
#[path = "plugin_id_tests.rs"]
mod tests;
