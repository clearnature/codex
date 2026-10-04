use super::*;
use codex_i18n::current;
use codex_i18n::tr_with;
use codex_protocol::error::CodexErrorDetails;

pub(super) fn environment_selection_error(err: CodexErr) -> JSONRPCErrorError {
    match err.details() {
        CodexErrorDetails::InvalidRequest(message) => invalid_request(message.clone()),
        _ => internal_error(tr_with(
            current(),
            "failed to validate environment selections: {0}",
            &[&err.to_string()],
        )),
    }
}
