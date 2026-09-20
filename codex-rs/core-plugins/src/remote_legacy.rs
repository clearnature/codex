use crate::error_subtype::http_status_sub_error_type;
use crate::remote::RemotePluginServiceConfig;
use codex_http_client::RouteAwareRequestError;
use codex_i18n::current;
use codex_i18n::tr_with;
use codex_login::CodexAuth;
use codex_protocol::protocol::Product;
use http::Method;
use http::StatusCode;
use serde::Deserialize;
use std::time::Duration;
use url::Url;

const REMOTE_FEATURED_PLUGIN_FETCH_TIMEOUT: Duration = Duration::from_secs(10);
const REMOTE_PLUGIN_MUTATION_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RemotePluginMutationResponse {
    pub id: String,
    pub enabled: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum RemotePluginMutationError {
    #[error("{}", tr_with(current(), "chatgpt authentication required for remote plugin mutation", &[]))]
    AuthRequired,

    #[error(
        "{}", tr_with(current(), "chatgpt authentication required for remote plugin mutation; api key auth is not supported", &[])
    )]
    UnsupportedAuthMode,

    #[error("{}", tr_with(current(), "failed to read auth token for remote plugin mutation: {0}", &[&_0.to_string()]))]
    AuthToken(#[source] std::io::Error),

    #[error("{}", tr_with(current(), "invalid chatgpt base url for remote plugin mutation: {0}", &[&_0.to_string()]))]
    InvalidBaseUrl(#[source] url::ParseError),

    #[error("{}", tr_with(current(), "chatgpt base url cannot be used for plugin mutation", &[]))]
    InvalidBaseUrlPath,

    #[error("{}", tr_with(current(), "failed to send remote plugin mutation request to {0}: {1}", &[url.as_str(), &source.to_string()]))]
    Request {
        url: String,
        #[source]
        source: RouteAwareRequestError,
    },

    #[error("{}", tr_with(current(), "remote plugin mutation failed with status {0} from {1}: {2}", &[&status.to_string(), url.as_str(), body.as_str()]))]
    UnexpectedStatus {
        url: String,
        status: StatusCode,
        body: String,
    },

    #[error("{}", tr_with(current(), "failed to parse remote plugin mutation response from {0}: {1}", &[url.as_str(), &source.to_string()]))]
    Decode {
        url: String,
        #[source]
        source: serde_json::Error,
    },

    #[error(
        "{}", tr_with(current(), "remote plugin mutation returned unexpected plugin id: expected `{0}`, got `{1}`", &[expected.as_str(), actual.as_str()])
    )]
    UnexpectedPluginId { expected: String, actual: String },

    #[error(
        "{}", tr_with(current(), "remote plugin mutation returned unexpected enabled state for `{0}`: expected {1}, got {2}", &[plugin_id.as_str(), &expected_enabled.to_string(), &actual_enabled.to_string()])
    )]
    UnexpectedEnabledState {
        plugin_id: String,
        expected_enabled: bool,
        actual_enabled: bool,
    },
}

impl RemotePluginMutationError {
    pub(crate) fn sub_error_type(&self) -> Option<String> {
        match self {
            Self::UnexpectedStatus { status, .. } => {
                Some(http_status_sub_error_type(*status).to_string())
            }
            Self::AuthRequired
            | Self::UnsupportedAuthMode
            | Self::AuthToken(_)
            | Self::InvalidBaseUrl(_)
            | Self::InvalidBaseUrlPath
            | Self::Request { .. }
            | Self::Decode { .. }
            | Self::UnexpectedPluginId { .. }
            | Self::UnexpectedEnabledState { .. } => None,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum RemotePluginFetchError {
    #[error("{}", tr_with(current(), "invalid chatgpt base url for remote featured plugin request: {0}", &[&_0.to_string()]))]
    InvalidBaseUrl(#[source] url::ParseError),

    #[error("{}", tr_with(current(), "failed to send remote featured plugin request to {0}: {1}", &[url.as_str(), &source.to_string()]))]
    Request {
        url: String,
        #[source]
        source: RouteAwareRequestError,
    },

    #[error("{}", tr_with(current(), "remote featured plugin request to {0} failed with status {1}: {2}", &[url.as_str(), &status.to_string(), body.as_str()]))]
    UnexpectedStatus {
        url: String,
        status: StatusCode,
        body: String,
    },

    #[error("{}", tr_with(current(), "failed to parse remote featured plugin response from {0}: {1}", &[url.as_str(), &source.to_string()]))]
    Decode {
        url: String,
        #[source]
        source: serde_json::Error,
    },
}

pub async fn fetch_remote_featured_plugin_ids(
    config: &RemotePluginServiceConfig,
    auth: Option<&CodexAuth>,
    product: Option<Product>,
) -> Result<Vec<String>, RemotePluginFetchError> {
    let base_url = config.chatgpt_base_url.trim_end_matches('/');
    let mut url = Url::parse(&format!("{base_url}/plugins/featured"))
        .map_err(RemotePluginFetchError::InvalidBaseUrl)?;
    url.query_pairs_mut().append_pair(
        "platform",
        product.unwrap_or(Product::Codex).to_app_platform(),
    );
    let url = url.to_string();
    let mut request = config
        .http_request(Method::GET, &url)
        .timeout(REMOTE_FEATURED_PLUGIN_FETCH_TIMEOUT);

    if let Some(auth) = auth.filter(|auth| auth.uses_codex_backend()) {
        request =
            request.headers(codex_model_provider::auth_provider_from_auth(auth).to_auth_headers());
    }

    let response = request
        .send()
        .await
        .map_err(|source| RemotePluginFetchError::Request {
            url: url.clone(),
            source,
        })?;
    let status = response.status();
    let body = response.text().await.unwrap_or_default();
    if !status.is_success() {
        return Err(RemotePluginFetchError::UnexpectedStatus { url, status, body });
    }

    serde_json::from_str(&body).map_err(|source| RemotePluginFetchError::Decode {
        url: url.clone(),
        source,
    })
}

pub async fn enable_remote_plugin(
    config: &RemotePluginServiceConfig,
    auth: Option<&CodexAuth>,
    plugin_id: &str,
) -> Result<(), RemotePluginMutationError> {
    post_remote_plugin_mutation(config, auth, plugin_id, "enable").await?;
    Ok(())
}

pub async fn uninstall_remote_plugin(
    config: &RemotePluginServiceConfig,
    auth: Option<&CodexAuth>,
    plugin_id: &str,
) -> Result<(), RemotePluginMutationError> {
    post_remote_plugin_mutation(config, auth, plugin_id, "uninstall").await?;
    Ok(())
}

fn ensure_codex_backend_auth(
    auth: Option<&CodexAuth>,
) -> Result<&CodexAuth, RemotePluginMutationError> {
    let Some(auth) = auth else {
        return Err(RemotePluginMutationError::AuthRequired);
    };
    if !auth.uses_codex_backend() {
        return Err(RemotePluginMutationError::UnsupportedAuthMode);
    }
    Ok(auth)
}

async fn post_remote_plugin_mutation(
    config: &RemotePluginServiceConfig,
    auth: Option<&CodexAuth>,
    plugin_id: &str,
    action: &str,
) -> Result<RemotePluginMutationResponse, RemotePluginMutationError> {
    let auth = ensure_codex_backend_auth(auth)?;
    let url = remote_plugin_mutation_url(config, plugin_id, action)?;
    let request = config
        .http_request(Method::POST, &url)
        .timeout(REMOTE_PLUGIN_MUTATION_TIMEOUT)
        .headers(codex_model_provider::auth_provider_from_auth(auth).to_auth_headers());

    let response = request
        .send()
        .await
        .map_err(|source| RemotePluginMutationError::Request {
            url: url.clone(),
            source,
        })?;
    let status = response.status();
    let body = response.text().await.unwrap_or_default();
    if !status.is_success() {
        return Err(RemotePluginMutationError::UnexpectedStatus { url, status, body });
    }

    let parsed: RemotePluginMutationResponse =
        serde_json::from_str(&body).map_err(|source| RemotePluginMutationError::Decode {
            url: url.clone(),
            source,
        })?;
    let expected_enabled = action == "enable";
    if parsed.id != plugin_id {
        return Err(RemotePluginMutationError::UnexpectedPluginId {
            expected: plugin_id.to_string(),
            actual: parsed.id,
        });
    }
    if parsed.enabled != expected_enabled {
        return Err(RemotePluginMutationError::UnexpectedEnabledState {
            plugin_id: plugin_id.to_string(),
            expected_enabled,
            actual_enabled: parsed.enabled,
        });
    }

    Ok(parsed)
}

fn remote_plugin_mutation_url(
    config: &RemotePluginServiceConfig,
    plugin_id: &str,
    action: &str,
) -> Result<String, RemotePluginMutationError> {
    let mut url = Url::parse(config.chatgpt_base_url.trim_end_matches('/'))
        .map_err(RemotePluginMutationError::InvalidBaseUrl)?;
    {
        let mut segments = url
            .path_segments_mut()
            .map_err(|()| RemotePluginMutationError::InvalidBaseUrlPath)?;
        segments.pop_if_empty();
        segments.push("plugins");
        segments.push(plugin_id);
        segments.push(action);
    }
    Ok(url.to_string())
}
