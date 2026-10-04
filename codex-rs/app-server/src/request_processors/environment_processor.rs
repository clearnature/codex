use super::*;
use codex_i18n::current;
use codex_i18n::tr_with;
use std::time::Duration;

#[derive(Clone)]
pub(crate) struct EnvironmentRequestProcessor {
    environment_manager: Arc<EnvironmentManager>,
}

impl EnvironmentRequestProcessor {
    pub(crate) fn new(environment_manager: Arc<EnvironmentManager>) -> Self {
        Self {
            environment_manager,
        }
    }

    pub(crate) async fn environment_add(
        &self,
        params: EnvironmentAddParams,
    ) -> Result<Option<ClientResponsePayload>, JSONRPCErrorError> {
        self.environment_manager
            .upsert_environment(
                params.environment_id,
                params.exec_server_url,
                params.connect_timeout_ms.map(Duration::from_millis),
            )
            .map_err(|err| invalid_request(err.to_string()))?;
        Ok(Some(EnvironmentAddResponse {}.into()))
    }

    pub(crate) async fn environment_info(
        &self,
        params: EnvironmentInfoParams,
    ) -> Result<Option<ClientResponsePayload>, JSONRPCErrorError> {
        let environment_id = params.environment_id;
        let environment = self
            .environment_manager
            .get_environment(&environment_id)
            .ok_or_else(|| {
                invalid_request(tr_with(
                    current(),
                    "unknown environment id `{0}`",
                    &[&environment_id],
                ))
            })?;
        let info = environment.force_info().await.map_err(|err| {
            internal_error(tr_with(
                current(),
                "failed to get info for environment `{0}`: {1}",
                &[&environment_id, &err.to_string()],
            ))
        })?;
        Ok(Some(
            EnvironmentInfoResponse {
                shell: EnvironmentShellInfo {
                    name: info.shell.name,
                    path: info.shell.path,
                },
                cwd: info.cwd,
            }
            .into(),
        ))
    }

    pub(crate) async fn environment_status(
        &self,
        params: EnvironmentStatusParams,
    ) -> Result<Option<ClientResponsePayload>, JSONRPCErrorError> {
        let environment_id = params.environment_id;
        let (status, error) = match self
            .environment_manager
            .get_environment_status(&environment_id)
            .await
        {
            Some(EnvironmentObservedStatus::Ready) => (EnvironmentStatusKind::Ready, None),
            Some(EnvironmentObservedStatus::Pending) => (EnvironmentStatusKind::Pending, None),
            Some(EnvironmentObservedStatus::Disconnected { error }) => {
                (EnvironmentStatusKind::Disconnected, Some(error))
            }
            None => (
                EnvironmentStatusKind::Unknown,
                Some(tr_with(
                    current(),
                    "unknown environment id `{0}`",
                    &[&environment_id],
                )),
            ),
        };
        Ok(Some(EnvironmentStatusResponse { status, error }.into()))
    }
}
