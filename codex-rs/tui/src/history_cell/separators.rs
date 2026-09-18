//! Turn separators and runtime-metrics labels for transcript history.

use super::*;
use codex_i18n::current;
use codex_i18n::tr;
use codex_i18n::tr_with;

#[derive(Debug)]
/// A visual divider between turns, optionally showing how long the assistant "worked for".
///
/// This separator is only emitted for turns that performed concrete work (e.g., running commands,
/// applying patches, making MCP tool calls), so purely conversational turns do not show an empty
/// divider.
pub struct FinalMessageSeparator {
    elapsed_seconds: Option<u64>,
    runtime_metrics: Option<RuntimeMetricsSummary>,
}
impl FinalMessageSeparator {
    /// Creates a separator; completed turns should pass protocol turn duration when available.
    pub(crate) fn new(
        elapsed_seconds: Option<u64>,
        runtime_metrics: Option<RuntimeMetricsSummary>,
    ) -> Self {
        Self {
            elapsed_seconds,
            runtime_metrics,
        }
    }
}
impl HistoryCell for FinalMessageSeparator {
    fn display_lines(&self, width: u16) -> Vec<Line<'static>> {
        let mut label_parts = Vec::new();
        if let Some(elapsed_seconds) = self
            .elapsed_seconds
            .filter(|seconds| *seconds > 60)
            .map(crate::status_indicator_widget::fmt_elapsed_compact)
        {
            label_parts.push(tr_with(current(), "Worked for {0}", &[&elapsed_seconds]));
        }
        if let Some(metrics_label) = self.runtime_metrics.and_then(runtime_metrics_label) {
            label_parts.push(metrics_label);
        }

        if label_parts.is_empty() {
            return vec![Line::from_iter(["─".repeat(width as usize).dim()])];
        }

        let label = format!("─ {} ─", label_parts.join(" • "));
        let (label, _suffix, label_width) = take_prefix_by_width(&label, width as usize);
        vec![
            Line::from_iter([
                label,
                "─".repeat((width as usize).saturating_sub(label_width)),
            ])
            .dim(),
        ]
    }

    fn raw_lines(&self) -> Vec<Line<'static>> {
        let mut label_parts = Vec::new();
        if let Some(elapsed_seconds) = self
            .elapsed_seconds
            .filter(|seconds| *seconds > 60)
            .map(crate::status_indicator_widget::fmt_elapsed_compact)
        {
            label_parts.push(tr_with(current(), "Worked for {0}", &[&elapsed_seconds]));
        }
        if let Some(metrics_label) = self.runtime_metrics.and_then(runtime_metrics_label) {
            label_parts.push(metrics_label);
        }
        if label_parts.is_empty() {
            Vec::new()
        } else {
            vec![Line::from(label_parts.join(" • "))]
        }
    }
}

pub(crate) fn runtime_metrics_label(summary: RuntimeMetricsSummary) -> Option<String> {
    let mut parts = Vec::new();
    if summary.tool_calls.count > 0 {
        let duration = format_duration_ms(summary.tool_calls.duration_ms);
        let calls = pluralize(
            summary.tool_calls.count,
            tr(current(), "call"),
            tr(current(), "calls"),
        );
        parts.push(tr_with(
            current(),
            "Local tools: {0} {1} ({2})",
            &[
                &summary.tool_calls.count.to_string(),
                calls,
                duration.as_str(),
            ],
        ));
    }
    if summary.api_calls.count > 0 {
        let duration = format_duration_ms(summary.api_calls.duration_ms);
        let calls = pluralize(
            summary.api_calls.count,
            tr(current(), "call"),
            tr(current(), "calls"),
        );
        parts.push(tr_with(
            current(),
            "Inference: {0} {1} ({2})",
            &[
                &summary.api_calls.count.to_string(),
                calls,
                duration.as_str(),
            ],
        ));
    }
    if summary.websocket_calls.count > 0 {
        let duration = format_duration_ms(summary.websocket_calls.duration_ms);
        parts.push(tr_with(
            current(),
            "WebSocket: {0} events send ({1})",
            &[
                &summary.websocket_calls.count.to_string(),
                duration.as_str(),
            ],
        ));
    }
    if summary.streaming_events.count > 0 {
        let duration = format_duration_ms(summary.streaming_events.duration_ms);
        let stream_label = pluralize(
            summary.streaming_events.count,
            tr(current(), "Stream"),
            tr(current(), "Streams"),
        );
        let events = pluralize(
            summary.streaming_events.count,
            tr(current(), "event"),
            tr(current(), "events"),
        );
        parts.push(format!(
            "{stream_label}: {} {events} ({duration})",
            summary.streaming_events.count
        ));
    }
    if summary.websocket_events.count > 0 {
        let duration = format_duration_ms(summary.websocket_events.duration_ms);
        parts.push(tr_with(
            current(),
            "{0} events received ({1})",
            &[
                &summary.websocket_events.count.to_string(),
                duration.as_str(),
            ],
        ));
    }
    if summary.responses_api_overhead_ms > 0 {
        let duration = format_duration_ms(summary.responses_api_overhead_ms);
        parts.push(tr_with(
            current(),
            "Responses API overhead: {0}",
            &[duration.as_str()],
        ));
    }
    if summary.responses_api_inference_time_ms > 0 {
        let duration = format_duration_ms(summary.responses_api_inference_time_ms);
        parts.push(tr_with(
            current(),
            "Responses API inference: {0}",
            &[duration.as_str()],
        ));
    }
    if summary.responses_api_engine_iapi_ttft_ms > 0
        || summary.responses_api_engine_service_ttft_ms > 0
    {
        let mut ttft_parts = Vec::new();
        if summary.responses_api_engine_iapi_ttft_ms > 0 {
            let duration = format_duration_ms(summary.responses_api_engine_iapi_ttft_ms);
            ttft_parts.push(tr_with(current(), "{0} (iapi)", &[duration.as_str()]));
        }
        if summary.responses_api_engine_service_ttft_ms > 0 {
            let duration = format_duration_ms(summary.responses_api_engine_service_ttft_ms);
            ttft_parts.push(tr_with(current(), "{0} (service)", &[duration.as_str()]));
        }
        parts.push(tr_with(current(), "TTFT: {0}", &[&ttft_parts.join(" ")]));
    }
    if summary.responses_api_engine_iapi_tbt_ms > 0.0
        || summary.responses_api_engine_service_tbt_ms > 0.0
    {
        let mut tbt_parts = Vec::new();
        if summary.responses_api_engine_iapi_tbt_ms > 0.0 {
            let duration =
                format_duration_ms(summary.responses_api_engine_iapi_tbt_ms.round() as u64);
            tbt_parts.push(tr_with(current(), "{0} (iapi)", &[duration.as_str()]));
        }
        if summary.responses_api_engine_service_tbt_ms > 0.0 {
            let duration =
                format_duration_ms(summary.responses_api_engine_service_tbt_ms.round() as u64);
            tbt_parts.push(tr_with(current(), "{0} (service)", &[duration.as_str()]));
        }
        parts.push(tr_with(current(), "TBT: {0}", &[&tbt_parts.join(" ")]));
    }
    if parts.is_empty() {
        None
    } else {
        Some(parts.join(" • "))
    }
}

fn format_duration_ms(duration_ms: u64) -> String {
    if duration_ms >= 1_000 {
        let seconds = duration_ms as f64 / 1_000.0;
        format!("{seconds:.1}s")
    } else {
        format!("{duration_ms}ms")
    }
}

fn pluralize(count: u64, singular: &'static str, plural: &'static str) -> &'static str {
    if count == 1 { singular } else { plural }
}
