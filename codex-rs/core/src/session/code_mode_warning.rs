use codex_features::Feature;
use codex_features::Features;
use codex_i18n::current;
use codex_i18n::tr_with;
use codex_protocol::openai_models::ModelInfo;

pub(super) fn unsupported_code_mode_warning(
    model_info: &ModelInfo,
    features: &Features,
) -> Option<String> {
    let code_mode_enabled =
        features.enabled(Feature::CodeMode) || features.enabled(Feature::CodeModeOnly);
    if !code_mode_enabled
        || model_info.tool_mode.is_some()
        || model_info.used_fallback_model_metadata
    {
        return None;
    }

    Some(tr_with(
        current(),
        "Code Mode is enabled in configuration, but model `{0}` does not advertise Code Mode support. This may degrade model performance. Disable `features.code_mode` and `features.code_mode_only`, or select a model whose metadata enables Code Mode.",
        &[model_info.slug.as_str()],
    ))
}

#[cfg(test)]
#[path = "code_mode_warning_tests.rs"]
mod tests;
