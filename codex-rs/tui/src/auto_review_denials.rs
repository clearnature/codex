use codex_i18n::current;
use codex_i18n::tr;
use codex_i18n::tr_with;
use std::collections::VecDeque;

use codex_protocol::approvals::GuardianAssessmentAction;
use codex_protocol::approvals::GuardianAssessmentEvent;
use codex_protocol::approvals::GuardianAssessmentStatus;

const MAX_RECENT_DENIALS: usize = 10;

#[derive(Debug, Default)]
pub(crate) struct RecentAutoReviewDenials {
    entries: VecDeque<GuardianAssessmentEvent>,
}

impl RecentAutoReviewDenials {
    pub(crate) fn push(&mut self, event: GuardianAssessmentEvent) {
        if event.status != GuardianAssessmentStatus::Denied {
            return;
        }

        self.entries.retain(|entry| entry.id != event.id);
        self.entries.push_front(event);
        self.entries.truncate(MAX_RECENT_DENIALS);
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub(crate) fn entries(&self) -> impl Iterator<Item = &GuardianAssessmentEvent> {
        self.entries.iter()
    }

    pub(crate) fn take(&mut self, id: &str) -> Option<GuardianAssessmentEvent> {
        let idx = self.entries.iter().position(|entry| entry.id == id)?;
        self.entries.remove(idx)
    }
}

pub(crate) fn action_summary(action: &GuardianAssessmentAction) -> String {
    match action {
        GuardianAssessmentAction::Command { command, .. } => command.clone(),
        GuardianAssessmentAction::Execve { program, argv, .. } => {
            let command = if argv.is_empty() {
                vec![program.clone()]
            } else {
                argv.clone()
            };
            shlex::try_join(command.iter().map(String::as_str))
                .unwrap_or_else(|_| command.join(" "))
        }
        GuardianAssessmentAction::WriteStdin {
            process_id, stdin, ..
        } => {
            let stdin = crate::text_formatting::truncate_text(
                &format!("{stdin:?}"),
                /*max_graphemes*/ 80,
            );
            tr_with(
                current(),
                "send input to terminal {0}: {1}",
                &[&process_id.to_string(), &stdin],
            )
        }
        GuardianAssessmentAction::ApplyPatch { files, .. } => {
            if files.len() == 1 {
                tr_with(
                    current(),
                    "apply_patch touching {0}",
                    &[&files[0].render_for_ui().to_string()],
                )
            } else {
                tr_with(
                    current(),
                    "apply_patch touching {0} files",
                    &[&files.len().to_string()],
                )
            }
        }
        GuardianAssessmentAction::NetworkAccess { target, .. } => {
            tr_with(current(), "network access to {0}", &[target])
        }
        GuardianAssessmentAction::McpToolCall {
            server,
            tool_name,
            connector_name,
            ..
        } => {
            let label = connector_name.as_deref().unwrap_or(server.as_str());
            tr_with(current(), "MCP {0} on {1}", &[tool_name, &label])
        }
        GuardianAssessmentAction::RequestPermissions { reason, .. } => reason
            .as_deref()
            .map(|reason| tr_with(current(), "permission request: {0}", &[reason]))
            .unwrap_or_else(|| tr(current(), "permission request").to_string()),
    }
}

#[cfg(test)]
mod tests {
    use codex_protocol::approvals::GuardianCommandSource;
    use codex_utils_absolute_path::test_support::PathBufExt;
    use codex_utils_absolute_path::test_support::test_path_buf;
    use pretty_assertions::assert_eq;

    use super::*;

    fn denied_event(id: usize) -> GuardianAssessmentEvent {
        GuardianAssessmentEvent {
            review_reason: None,
            id: format!("review-{id}"),
            target_item_id: None,
            plugin_id: None,
            script_path: None,
            turn_id: "turn-1".to_string(),
            started_at_ms: 0,
            completed_at_ms: Some(1),
            status: GuardianAssessmentStatus::Denied,
            risk_level: None,
            user_authorization: None,
            rationale: Some(format!("rationale {id}")),
            decision_source: None,
            action: GuardianAssessmentAction::Command {
                source: GuardianCommandSource::Shell,
                command: format!("rm -rf /tmp/test-{id}"),
                cwd: test_path_buf("/tmp").abs().into(),
            },
        }
    }

    #[test]
    fn keeps_only_ten_most_recent_denials() {
        let mut denials = RecentAutoReviewDenials::default();
        for id in 0..12 {
            denials.push(denied_event(id));
        }

        let ids = denials
            .entries()
            .map(|entry| entry.id.as_str())
            .collect::<Vec<_>>();
        assert_eq!(
            ids,
            vec![
                "review-11",
                "review-10",
                "review-9",
                "review-8",
                "review-7",
                "review-6",
                "review-5",
                "review-4",
                "review-3",
                "review-2",
            ]
        );
    }
}
