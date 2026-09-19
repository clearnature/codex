use codex_i18n::current;
use codex_i18n::tr_with;
use std::io::ErrorKind;
use std::path::Path;

use crate::rollout::SESSIONS_SUBDIR;
use codex_protocol::error::CodexErr;
use codex_thread_store::ThreadStoreError;

pub(crate) fn map_session_init_error(err: &anyhow::Error, codex_home: &Path) -> CodexErr {
    if let Some(store_error) = err
        .chain()
        .find_map(|cause| cause.downcast_ref::<ThreadStoreError>())
    {
        match store_error {
            ThreadStoreError::Unsupported { operation } => {
                return CodexErr::UnsupportedOperation(tr_with(
                    current(),
                    "{0} is not supported yet",
                    &[operation],
                ));
            }
            ThreadStoreError::Conflict { message } => {
                return CodexErr::InvalidRequest(message.clone());
            }
            ThreadStoreError::ThreadNotFound { .. }
            | ThreadStoreError::InvalidRequest { .. }
            | ThreadStoreError::Internal { .. } => {}
        }
    }

    if let Some(mapped) = err
        .chain()
        .filter_map(|cause| cause.downcast_ref::<std::io::Error>())
        .find_map(|io_err| map_rollout_io_error(io_err, codex_home))
    {
        return mapped;
    }

    CodexErr::Fatal(tr_with(
        current(),
        "Failed to initialize session: {0}",
        &[&format!("{err:#}")],
    ))
}

fn map_rollout_io_error(io_err: &std::io::Error, codex_home: &Path) -> Option<CodexErr> {
    let sessions_dir = codex_home.join(SESSIONS_SUBDIR);
    let hint = match io_err.kind() {
        ErrorKind::PermissionDenied => tr_with(
            current(),
            "Codex cannot access session files at {0} (permission denied). If sessions were created using sudo, fix ownership: sudo chown -R $(whoami) {1}",
            &[
                &sessions_dir.display().to_string(),
                &codex_home.display().to_string(),
            ],
        ),
        ErrorKind::NotFound => tr_with(
            current(),
            "Session storage missing at {0}. Create the directory or choose a different Codex home.",
            &[&sessions_dir.display().to_string()],
        ),
        ErrorKind::AlreadyExists => tr_with(
            current(),
            "Session storage path {0} is blocked by an existing file. Remove or rename it so Codex can create sessions.",
            &[&sessions_dir.display().to_string()],
        ),
        ErrorKind::InvalidData => tr_with(
            current(),
            "Session data under {0} looks corrupt or unreadable. Clearing the sessions directory may help (this will remove saved threads).",
            &[&sessions_dir.display().to_string()],
        ),
        ErrorKind::IsADirectory | ErrorKind::NotADirectory => tr_with(
            current(),
            "Session storage path {0} has an unexpected type. Ensure it is a directory Codex can use for session files.",
            &[&sessions_dir.display().to_string()],
        ),
        _ => return None,
    };

    Some(CodexErr::Fatal(tr_with(
        current(),
        "{0} (underlying error: {1})",
        &[&hint, &io_err.to_string()],
    )))
}
