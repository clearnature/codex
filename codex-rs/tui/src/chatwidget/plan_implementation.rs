use codex_i18n::current;
use codex_i18n::tr;
use codex_i18n::tr_with;
use codex_protocol::config_types::CollaborationModeMask;

use crate::app_event::AppEvent;
use crate::bottom_pane::SelectionAction;
use crate::bottom_pane::SelectionItem;
use crate::bottom_pane::SelectionViewParams;
use crate::bottom_pane::popup_consts::standard_popup_hint_line;

pub(super) fn plan_implementation_title() -> &'static str {
    tr(current(), "Implement this plan?")
}

fn plan_implementation_yes() -> &'static str {
    tr(current(), "Yes, implement this plan")
}

fn plan_implementation_clear_context() -> &'static str {
    tr(current(), "Yes, clear context and implement")
}

fn plan_implementation_no() -> &'static str {
    tr(current(), "No, stay in Plan mode")
}
pub(super) const PLAN_IMPLEMENTATION_CODING_MESSAGE: &str = "Implement the plan.";
pub(super) const PLAN_IMPLEMENTATION_CLEAR_CONTEXT_PREFIX: &str = concat!(
    "A previous agent produced the plan below to accomplish the user's task. ",
    "Implement the plan in a fresh context. Treat the plan as the source of ",
    "user intent, re-read files as needed, and carry the work through ",
    "implementation and verification."
);
pub(super) fn plan_implementation_default_unavailable() -> &'static str {
    tr(current(), "Default mode unavailable")
}

pub(super) fn plan_implementation_no_approved_plan() -> &'static str {
    tr(current(), "No approved plan available")
}

/// Builds the confirmation prompt shown after a plan is approved in Plan mode.
///
/// The optional usage label is already phrased for display, such as `89% used`
/// or `123K used`. This module only decides where that label belongs in the
/// decision copy so action wiring stays separate from token accounting.
pub(super) fn selection_view_params(
    default_mask: Option<CollaborationModeMask>,
    plan_markdown: Option<&str>,
    clear_context_usage_label: Option<&str>,
) -> SelectionViewParams {
    let (implement_actions, implement_disabled_reason) = match default_mask.clone() {
        Some(mask) => {
            let user_text = PLAN_IMPLEMENTATION_CODING_MESSAGE.to_string();
            let actions: Vec<SelectionAction> = vec![Box::new(move |tx| {
                tx.send(AppEvent::SubmitUserMessageWithMode {
                    text: user_text.clone(),
                    collaboration_mode: mask.clone(),
                });
            })];
            (actions, None)
        }
        None => (
            Vec::new(),
            Some(plan_implementation_default_unavailable().to_string()),
        ),
    };

    let (clear_context_actions, clear_context_disabled_reason) = match (default_mask, plan_markdown)
    {
        (None, _) => (
            Vec::new(),
            Some(plan_implementation_default_unavailable().to_string()),
        ),
        (Some(_), Some(plan_markdown)) if !plan_markdown.trim().is_empty() => {
            let user_text =
                format!("{PLAN_IMPLEMENTATION_CLEAR_CONTEXT_PREFIX}\n\n{plan_markdown}");
            let actions: Vec<SelectionAction> = vec![Box::new(move |tx| {
                tx.send(AppEvent::ClearUiAndSubmitUserMessage {
                    text: user_text.clone(),
                });
            })];
            (actions, None)
        }
        (Some(_), _) => (
            Vec::new(),
            Some(plan_implementation_no_approved_plan().to_string()),
        ),
    };

    let clear_context_description = clear_context_usage_label.map_or_else(
        || tr(current(), "Fresh thread with this plan.").to_string(),
        |label| tr_with(current(), "Fresh thread. Context: {0}.", &[label]),
    );

    SelectionViewParams {
        title: Some(plan_implementation_title().to_string()),
        subtitle: None,
        footer_hint: Some(standard_popup_hint_line()),
        items: vec![
            SelectionItem {
                name: plan_implementation_yes().to_string(),
                description: Some(tr(current(), "Switch to Default and start coding.").to_string()),
                selected_description: None,
                is_current: false,
                actions: implement_actions,
                disabled_reason: implement_disabled_reason,
                dismiss_on_select: true,
                ..Default::default()
            },
            SelectionItem {
                name: plan_implementation_clear_context().to_string(),
                description: Some(clear_context_description),
                selected_description: None,
                is_current: false,
                actions: clear_context_actions,
                disabled_reason: clear_context_disabled_reason,
                dismiss_on_select: true,
                ..Default::default()
            },
            SelectionItem {
                name: plan_implementation_no().to_string(),
                description: Some(tr(current(), "Continue planning with the model.").to_string()),
                selected_description: None,
                is_current: false,
                actions: Vec::new(),
                dismiss_on_select: true,
                ..Default::default()
            },
        ],
        ..Default::default()
    }
}
