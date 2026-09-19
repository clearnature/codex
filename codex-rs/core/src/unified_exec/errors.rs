use codex_i18n::current;
use codex_i18n::tr;
use codex_i18n::tr_with;
use codex_protocol::exec_output::ExecToolCallOutput;
use codex_utils_path_uri::PathUri;
use std::num::NonZeroUsize;
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum UnifiedExecError {
    #[error("{}", tr_with(current(), "Failed to create unified exec process: {0}", &[message]))]
    CreateProcess { message: String },
    #[error("{}", tr_with(current(), "Unified exec process failed: {0}", &[message]))]
    ProcessFailed { message: String },
    // The model is trained on `session_id`, but internally we track a `process_id`.
    #[error("{}", tr_with(current(), "Unknown process id {0}", &[&process_id.to_string()]))]
    UnknownProcessId { process_id: i32 },
    #[error("{}", tr_with(current(), "stdin approval failed: {0}", &[&format!("{_0:?}")]))]
    StdinApproval(crate::tools::sandboxing::ToolError),
    #[error("{}", tr(current(), "failed to write to stdin"))]
    WriteToStdin,
    #[error(
        "{}",
        tr(
            current(),
            "stdin is closed for this session; rerun exec_command with tty=true to keep stdin open"
        )
    )]
    StdinClosed,
    #[error("{}", tr(current(), "missing command line for unified exec request"))]
    MissingCommandLine,
    #[error("{}", tr_with(current(), "Command denied by sandbox: {0}", &[message]))]
    SandboxDenied {
        message: String,
        output: ExecToolCallOutput,
        original_token_count: Option<usize>,
        output_omitted_bytes: Option<NonZeroUsize>,
    },
    #[error("{}", tr_with(current(), "{0} is not valid on {1}", &[&path.to_string(), std::env::consts::OS]))]
    ForeignPath { path: PathUri },
}

impl UnifiedExecError {
    pub(crate) fn create_process(message: String) -> Self {
        Self::CreateProcess { message }
    }

    pub(crate) fn process_failed(message: String) -> Self {
        Self::ProcessFailed { message }
    }

    pub(crate) fn sandbox_denied(message: String, output: ExecToolCallOutput) -> Self {
        Self::SandboxDenied {
            message,
            output,
            original_token_count: None,
            output_omitted_bytes: None,
        }
    }

    pub(crate) fn with_output_collection_metadata(
        self,
        original_token_count: usize,
        output_omitted_bytes: Option<NonZeroUsize>,
    ) -> Self {
        match self {
            Self::SandboxDenied {
                message, output, ..
            } => Self::SandboxDenied {
                message,
                output,
                original_token_count: Some(original_token_count),
                output_omitted_bytes,
            },
            other => other,
        }
    }
}
