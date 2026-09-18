//! Desktop notification coalescing for `ChatWidget`.

use super::*;
use codex_i18n::current;
use codex_i18n::tr;
use codex_i18n::tr_with;

impl ChatWidget {
    pub(super) fn notify(&mut self, notification: Notification) {
        if !notification.allowed_for(&self.local_settings.tui.notification_settings.notifications) {
            return;
        }
        if let Some(existing) = self.pending_notification.as_ref()
            && existing.priority() > notification.priority()
        {
            return;
        }
        self.pending_notification = Some(notification);
        self.request_redraw();
    }

    pub(crate) fn maybe_post_pending_notification(&mut self, tui: &mut crate::tui::Tui) {
        if let Some(notif) = self.pending_notification.take() {
            tui.notify(notif.display());
        }
    }
}

#[derive(Debug)]
pub(super) enum Notification {
    AgentTurnComplete { response: String },
    ExecApprovalRequested { command: String },
    EditApprovalRequested { cwd: PathBuf, changes: Vec<PathBuf> },
    ElicitationRequested { server_name: String },
    PlanModePrompt { title: String },
}

impl Notification {
    pub(super) fn display(&self) -> String {
        match self {
            Notification::AgentTurnComplete { response } => {
                Notification::agent_turn_preview(response)
                    .unwrap_or_else(|| tr(current(), "Agent turn complete").to_string())
            }
            Notification::ExecApprovalRequested { command } => tr_with(
                current(),
                "Approval requested: {0}",
                &[&truncate_text(command, /*max_graphemes*/ 30)],
            ),
            Notification::EditApprovalRequested { cwd, changes } => {
                let target = if changes.len() == 1 {
                    #[allow(clippy::unwrap_used)]
                    display_path_for(changes.first().unwrap(), cwd)
                } else {
                    tr_with(current(), "{0} files", &[&changes.len().to_string()])
                };
                tr_with(current(), "Codex wants to edit {0}", &[&target])
            }
            Notification::ElicitationRequested { server_name } => {
                tr_with(current(), "Approval requested by {0}", &[server_name])
            }
            Notification::PlanModePrompt { title } => {
                tr_with(current(), "Plan mode prompt: {0}", &[title])
            }
        }
    }

    fn type_name(&self) -> &str {
        match self {
            Notification::AgentTurnComplete { .. } => "agent-turn-complete",
            Notification::ExecApprovalRequested { .. }
            | Notification::EditApprovalRequested { .. }
            | Notification::ElicitationRequested { .. } => "approval-requested",
            Notification::PlanModePrompt { .. } => "plan-mode-prompt",
        }
    }

    fn priority(&self) -> u8 {
        match self {
            Notification::AgentTurnComplete { .. } => 0,
            Notification::ExecApprovalRequested { .. }
            | Notification::EditApprovalRequested { .. }
            | Notification::ElicitationRequested { .. }
            | Notification::PlanModePrompt { .. } => 1,
        }
    }

    pub(super) fn allowed_for(&self, settings: &Notifications) -> bool {
        match settings {
            Notifications::Enabled(enabled) => *enabled,
            Notifications::Custom(allowed) => allowed.iter().any(|a| a == self.type_name()),
        }
    }

    pub(super) fn agent_turn_preview(response: &str) -> Option<String> {
        let mut normalized = String::new();
        for part in response.split_whitespace() {
            if !normalized.is_empty() {
                normalized.push(' ');
            }
            normalized.push_str(part);
        }
        let trimmed = normalized.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(truncate_text(trimmed, AGENT_NOTIFICATION_PREVIEW_GRAPHEMES))
        }
    }

    pub(super) fn user_input_request_summary(
        questions: &[codex_app_server_protocol::ToolRequestUserInputQuestion],
    ) -> Option<String> {
        let first_question = questions.first()?;
        let summary = if first_question.header.trim().is_empty() {
            first_question.question.trim()
        } else {
            first_question.header.trim()
        };
        if summary.is_empty() {
            None
        } else {
            Some(truncate_text(summary, /*max_graphemes*/ 30))
        }
    }
}

const AGENT_NOTIFICATION_PREVIEW_GRAPHEMES: usize = 200;
