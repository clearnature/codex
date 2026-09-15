//! Goal summary for the bare `/goal` command.

use super::*;
use crate::goal_display::format_goal_elapsed_seconds;
use crate::goal_files;
use crate::status::format_tokens_compact;
use codex_i18n::current;
use codex_i18n::tr;
use codex_i18n::tr_with;

impl ChatWidget {
    pub(crate) fn show_goal_summary(&mut self, goal: AppThreadGoal) {
        self.add_plain_history_lines(goal_summary_lines(&goal));
    }

    pub(crate) fn show_goal_edit_prompt(&mut self, thread_id: ThreadId, goal: AppThreadGoal) {
        let tx = self.app_event_tx.clone();
        let status = edited_goal_status(goal.status);
        let token_budget = goal.token_budget;
        let view = CustomPromptView::new(
            tr(current(), "Edit goal").to_string(),
            tr(current(), "Type a goal objective and press Enter").to_string(),
            goal.objective,
            /*context_label*/ None,
            Box::new(move |objective: String| {
                tx.send(AppEvent::SetThreadGoalDraft {
                    thread_id,
                    draft: goal_files::GoalDraft {
                        objective,
                        ..Default::default()
                    },
                    mode: crate::app_event::ThreadGoalSetMode::UpdateExisting {
                        status,
                        token_budget,
                    },
                });
            }),
        );
        self.bottom_pane.show_text_prompt(view);
    }

    pub(crate) fn show_resume_paused_goal_prompt(
        &mut self,
        thread_id: ThreadId,
        objective: String,
    ) {
        let resume_actions: Vec<SelectionAction> = vec![Box::new(move |tx| {
            tx.send(AppEvent::SetThreadGoalStatus {
                thread_id,
                status: AppThreadGoalStatus::Active,
            });
        })];
        self.show_selection_view(SelectionViewParams {
            title: Some(tr(current(), "Resume paused goal?").to_string()),
            subtitle: Some(tr_with(current(), "Goal: {0}", &[&objective])),
            footer_hint: Some(standard_popup_hint_line()),
            initial_selected_idx: Some(0),
            items: vec![
                SelectionItem {
                    name: tr(current(), "Resume goal").to_string(),
                    description: Some(
                        tr(current(), "Mark it active and continue when idle").to_string(),
                    ),
                    actions: resume_actions,
                    dismiss_on_select: true,
                    ..Default::default()
                },
                SelectionItem {
                    name: tr(current(), "Leave paused").to_string(),
                    description: Some(
                        tr(current(), "Keep it paused; use /goal resume later").to_string(),
                    ),
                    dismiss_on_select: true,
                    ..Default::default()
                },
            ],
            ..Default::default()
        });
    }

    pub(crate) fn on_thread_goal_cleared(&mut self, thread_id: &str) {
        if self
            .thread_id
            .is_some_and(|active_thread_id| active_thread_id.to_string() == thread_id)
        {
            self.current_goal_status = None;
            self.update_collaboration_mode_indicator();
        }
    }
}

fn goal_summary_lines(goal: &AppThreadGoal) -> Vec<Line<'static>> {
    let mut lines = vec![
        Line::from("Goal".bold()),
        Line::from(vec![
            tr(current(), "Status: ").dim(),
            goal_status_label(goal.status).to_string().into(),
        ]),
        Line::from(vec![
            tr(current(), "Objective: ").dim(),
            goal.objective.clone().into(),
        ]),
        Line::from(vec![
            tr(current(), "Time used: ").dim(),
            format_goal_elapsed_seconds(goal.time_used_seconds).into(),
        ]),
        Line::from(vec![
            tr(current(), "Tokens used: ").dim(),
            format_tokens_compact(goal.tokens_used).into(),
        ]),
    ];
    if let Some(token_budget) = goal.token_budget {
        lines.push(Line::from(vec![
            tr(current(), "Token budget: ").dim(),
            format_tokens_compact(token_budget).into(),
        ]));
    }
    let command_hint = match goal.status {
        AppThreadGoalStatus::Active => {
            tr(current(), "Commands: /goal edit, /goal pause, /goal clear")
        }
        AppThreadGoalStatus::Paused
        | AppThreadGoalStatus::Blocked
        | AppThreadGoalStatus::UsageLimited => {
            tr(current(), "Commands: /goal edit, /goal resume, /goal clear")
        }
        AppThreadGoalStatus::BudgetLimited | AppThreadGoalStatus::Complete => {
            tr(current(), "Commands: /goal edit, /goal clear")
        }
    };
    lines.push(Line::default());
    lines.push(Line::from(command_hint.dim()));
    lines
}

fn goal_status_label(status: AppThreadGoalStatus) -> &'static str {
    match status {
        AppThreadGoalStatus::Active => tr(current(), "active"),
        AppThreadGoalStatus::Paused => tr(current(), "paused"),
        AppThreadGoalStatus::Blocked => tr(current(), "stalled"),
        AppThreadGoalStatus::UsageLimited => tr(current(), "usage limited"),
        AppThreadGoalStatus::BudgetLimited => tr(current(), "limited by budget"),
        AppThreadGoalStatus::Complete => tr(current(), "complete"),
    }
}

fn edited_goal_status(status: AppThreadGoalStatus) -> AppThreadGoalStatus {
    match status {
        AppThreadGoalStatus::Active => AppThreadGoalStatus::Active,
        AppThreadGoalStatus::Paused
        | AppThreadGoalStatus::Blocked
        | AppThreadGoalStatus::UsageLimited => status,
        AppThreadGoalStatus::BudgetLimited | AppThreadGoalStatus::Complete => {
            AppThreadGoalStatus::Active
        }
    }
}
