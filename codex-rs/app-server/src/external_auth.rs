use codex_i18n::current;
use codex_i18n::tr;
use codex_i18n::tr_with;
use std::sync::Arc;
use std::sync::RwLock;

use codex_app_server_protocol::ChatgptAuthTokensRefreshParams;
use codex_app_server_protocol::ChatgptAuthTokensRefreshReason;
use codex_app_server_protocol::ChatgptAuthTokensRefreshResponse;
use codex_app_server_protocol::ServerRequestPayload;
use codex_login::CodexAuth;
use codex_login::ExternalAuthFuture;
use codex_login::auth::ExternalAuth;
use codex_login::auth::ExternalAuthRefreshContext;
use codex_login::auth::ExternalAuthRefreshReason;
use tokio::time::Duration;
use tokio::time::timeout;

use crate::outgoing_message::OutgoingMessageSender;

const EXTERNAL_AUTH_REFRESH_TIMEOUT: Duration = Duration::from_secs(10);

pub(crate) struct ExternalAuthBridge {
    outgoing: Arc<OutgoingMessageSender>,
    auth: RwLock<CodexAuth>,
}

impl ExternalAuthBridge {
    pub(crate) fn new(outgoing: Arc<OutgoingMessageSender>, auth: CodexAuth) -> Self {
        Self {
            outgoing,
            auth: RwLock::new(auth),
        }
    }

    async fn refresh(&self, context: ExternalAuthRefreshContext) -> std::io::Result<CodexAuth> {
        let reason = match context.reason {
            ExternalAuthRefreshReason::Unauthorized => ChatgptAuthTokensRefreshReason::Unauthorized,
        };
        let params = ChatgptAuthTokensRefreshParams {
            reason,
            previous_account_id: context.previous_account_id,
        };

        let (request_id, rx) = self
            .outgoing
            .send_request(ServerRequestPayload::ChatgptAuthTokensRefresh(params))
            .await;
        let result = match timeout(EXTERNAL_AUTH_REFRESH_TIMEOUT, rx).await {
            Ok(result) => {
                let result = result.map_err(|err| {
                    std::io::Error::other(tr_with(
                        current(),
                        "auth refresh request canceled: {0}",
                        &[&err.to_string()],
                    ))
                })?;
                result.map_err(|err| {
                    // Don't log err.message because it may contain a token.
                    let code = err.code;
                    std::io::Error::other(tr_with(
                        current(),
                        "auth refresh request failed: code={0}",
                        &[&code.to_string()],
                    ))
                })?
            }
            Err(_) => {
                let _canceled = self.outgoing.cancel_request(&request_id).await;
                return Err(std::io::Error::other(tr_with(
                    current(),
                    "auth refresh request timed out after {0}s",
                    &[&EXTERNAL_AUTH_REFRESH_TIMEOUT.as_secs().to_string()],
                )));
            }
        };

        // Don't propagate parser error messages because they may contain a token.
        let response: ChatgptAuthTokensRefreshResponse = serde_json::from_value(result)
            .map_err(|_| std::io::Error::other(tr(current(), "invalid auth refresh response")))?;
        let auth = CodexAuth::from_external_chatgpt_tokens(
            response.access_token.as_str(),
            response.chatgpt_account_id.as_str(),
            response.chatgpt_plan_type.as_deref(),
        )
        .map_err(|err| {
            std::io::Error::new(
                err.kind(),
                tr(current(), "auth refresh returned invalid credentials"),
            )
        })?;
        *self.auth.write().map_err(|_| {
            std::io::Error::other(tr(current(), "external auth lock is poisoned"))
        })? = auth.clone();
        Ok(auth)
    }
}

impl ExternalAuth for ExternalAuthBridge {
    fn resolve(&self) -> ExternalAuthFuture<'_, CodexAuth> {
        Box::pin(async {
            self.auth
                .read()
                .map(|auth| auth.clone())
                .map_err(|_| std::io::Error::other(tr(current(), "external auth lock is poisoned")))
        })
    }

    fn refresh(&self, context: ExternalAuthRefreshContext) -> ExternalAuthFuture<'_, CodexAuth> {
        Box::pin(ExternalAuthBridge::refresh(self, context))
    }
}
