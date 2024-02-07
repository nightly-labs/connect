use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{
    errors::NightlyError,
    state::{SessionId, Sessions},
    structs::common::{AppMetadata, Network, SessionStatus, Version},
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct HttpGetSessionInfoRequest {
    #[serde(rename = "sessionId")]
    pub session_id: SessionId,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct HttpGetSessionInfoResponse {
    pub status: SessionStatus,
    pub persistent: bool,
    pub version: Version,
    pub network: Network,
    #[serde(rename = "appMetadata")]
    pub app_metadata: AppMetadata,
}
pub async fn get_session_info(
    State(sessions): State<Sessions>,
    Json(request): Json<HttpGetSessionInfoRequest>,
) -> Result<Json<HttpGetSessionInfoResponse>, (StatusCode, String)> {
    let sessions_read = sessions.read().await;
    let session_read = match sessions_read.get(&request.session_id) {
        Some(session) => session.read().await,
        None => {
            return Err((
                StatusCode::BAD_REQUEST,
                NightlyError::SessionDoesNotExist.to_string(),
            ))
        }
    };

    let response = HttpGetSessionInfoResponse {
        status: session_read.status.clone(),
        persistent: session_read.persistent,
        version: session_read.version.clone(),
        network: session_read.network.clone(),
        app_metadata: session_read.app_state.metadata.clone(),
    };
    return Ok(Json(response));
}
