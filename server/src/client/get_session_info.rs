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
    let sessions = sessions.read().await;
    let session = match sessions.get(&request.session_id) {
        Some(session) => session,
        None => {
            return Err((
                StatusCode::BAD_REQUEST,
                NightlyError::SessionDoesNotExist.to_string(),
            ))
        }
    };
    let response = HttpGetSessionInfoResponse {
        status: session.status.clone(),
        persistent: session.persistent,
        version: session.version.clone(),
        network: session.network.clone(),
        app_metadata: session.app_state.metadata.clone(),
    };
    return Ok(Json(response));
}
