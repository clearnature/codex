//! Destination and filename prompts for on-demand transcript exports.

use super::*;
use crate::app_event::TranscriptExportDestination;
use codex_i18n::current;
use codex_i18n::tr;
use codex_i18n::tr_with;

impl ChatWidget {
    pub(crate) fn copy_transcript_to_clipboard(&mut self, markdown: &str) {
        match crate::clipboard_copy::copy_to_clipboard(
            markdown,
            crate::clipboard_copy::CopyFormat::PlainText,
        ) {
            Ok(lease) => {
                self.clipboard_lease = lease;
                self.add_info_message(
                    tr(current(), "Copied conversation to clipboard").to_string(),
                    /*hint*/ None,
                );
            }
            Err(error) => self.add_error_message(tr_with(
                current(),
                "Copy failed: {0}",
                &[&error.to_string()],
            )),
        }
    }

    pub(super) fn show_transcript_export_popup(&mut self) {
        self.show_selection_view(SelectionViewParams {
            title: Some(tr(current(), "Export conversation").to_string()),
            subtitle: Some(tr(current(), "Save the complete conversation as Markdown").to_string()),
            footer_hint: Some(standard_popup_hint_line()),
            items: vec![
                SelectionItem {
                    name: tr(current(), "Copy to clipboard").to_string(),
                    description: Some(
                        tr(current(), "Copy the complete Markdown transcript").to_string(),
                    ),
                    is_disabled: cfg!(target_os = "android"),
                    actions: vec![Box::new(|tx| {
                        tx.send(AppEvent::ExportTranscript {
                            destination: TranscriptExportDestination::Clipboard,
                        });
                    })],
                    dismiss_on_select: true,
                    ..Default::default()
                },
                SelectionItem {
                    name: tr(current(), "Save to file").to_string(),
                    description: Some(tr(current(), "Choose a Markdown filename").to_string()),
                    actions: vec![Box::new(|tx| {
                        tx.send(AppEvent::OpenTranscriptExportFilePrompt);
                    })],
                    dismiss_on_select: true,
                    ..Default::default()
                },
            ],
            ..Default::default()
        });
        self.defer_input_until_settings_applied();
        self.request_redraw();
    }

    pub(crate) fn show_transcript_export_file_prompt(&mut self) {
        let tx = self.app_event_tx.clone();
        let filename = self.thread_id().map_or_else(
            || "codex-session.md".to_string(),
            |thread_id| format!("codex-session-{thread_id}.md"),
        );
        let view = CustomPromptView::new(
            tr(current(), "Save conversation").to_string(),
            tr(current(), "Type a filename and press Enter").to_string(),
            filename,
            /*context_label*/ None,
            Box::new(move |filename| {
                tx.send(AppEvent::ExportTranscript {
                    destination: TranscriptExportDestination::File(PathBuf::from(filename)),
                });
            }),
        );
        self.bottom_pane.show_text_prompt(view);
        self.request_redraw();
    }
}
