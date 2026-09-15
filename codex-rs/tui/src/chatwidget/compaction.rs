//! Live compaction status. Its wall clock is separate from the turn's running time,
//! and only a matching live completion contributes a duration to the transcript.

use super::*;
use codex_i18n::current;
use codex_i18n::tr;
use codex_i18n::tr_with;

/// Header shown while the context is being compacted.
///
/// A function rather than a `const` so the text can pass through `tr`, which a
/// `const` cannot do (see `docs/plan/i18n-design.md` §3.6).
pub(super) fn compaction_header() -> &'static str {
    tr(current(), "Compacting context")
}

/// One-line explanation shown under the header.
pub(super) fn compaction_details() -> &'static str {
    tr(current(), "Making room to continue.")
}

#[derive(Debug)]
pub(super) struct ActiveCompaction {
    pub(super) id: String,
    pub(super) started_at: Instant,
}

impl ChatWidget {
    pub(super) fn on_context_compaction_started(&mut self, id: String, elapsed: Duration) {
        if self
            .status_state
            .compaction
            .as_ref()
            .is_some_and(|active| active.id == id)
        {
            return;
        }
        self.flush_answer_stream_with_separator();
        let now = Instant::now();
        let started_at = now.checked_sub(elapsed).unwrap_or(now);
        self.status_state.compaction = Some(ActiveCompaction { id, started_at });
        self.bottom_pane.set_status_timer_origin(Some(started_at));
        self.bottom_pane.ensure_status_indicator();
        self.set_status_header(compaction_header().to_string());
    }

    pub(super) fn clear_context_compaction(&mut self) {
        if self.status_state.compaction.take().is_some() {
            self.bottom_pane
                .set_status_timer_origin(/*started_at*/ None);
            self.set_status_header(tr(current(), "Working").to_string());
        }
    }

    pub(super) fn on_context_compaction_completed(&mut self, id: &str, from_replay: bool) {
        let mut message = tr(current(), "Context compacted").to_string();
        if let Some(active) = self.status_state.compaction.as_ref()
            && active.id == id
        {
            if !from_replay {
                let elapsed = crate::status_indicator_widget::fmt_elapsed_compact(
                    active.started_at.elapsed().as_secs(),
                );
                message = tr_with(current(), "Context compacted · {0}", &[&elapsed]);
            }
            self.clear_context_compaction();
        }
        self.add_info_message(message, /*hint*/ None);
    }
}
