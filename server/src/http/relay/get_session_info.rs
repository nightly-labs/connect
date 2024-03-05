use crate::{
    errors::NightlyError,
    state::{SessionId, SessionToApp, SessionToAppMap, Sessions},
    structs::common::{AppMetadata, Network, SessionStatus, Version},
};
use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpGetSessionInfoRequest {
    pub session_id: SessionId,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpGetSessionInfoResponse {
    pub status: SessionStatus,
    pub persistent: bool,
    pub version: Version,
    pub network: Network,
    pub app_metadata: AppMetadata,
}

pub async fn get_session_info(
    State(sessions): State<Sessions>,
    State(session_to_app_map): State<SessionToAppMap>,
    Json(request): Json<HttpGetSessionInfoRequest>,
) -> Result<Json<HttpGetSessionInfoResponse>, (StatusCode, String)> {
    let app_id = match session_to_app_map.get_app_id(&request.session_id).await {
        Some(app_id) => app_id,
        None => {
            return Err((
                StatusCode::BAD_REQUEST,
                NightlyError::UnhandledInternalError.to_string(),
            ))
        }
    };

    let sessions_read = sessions.read().await;
    let app_sessions_read = match sessions_read.get(&app_id) {
        Some(session) => session.read().await,
        None => {
            return Err((
                StatusCode::BAD_REQUEST,
                NightlyError::SessionDoesNotExist.to_string(),
            ))
        }
    };

    let session_read = match app_sessions_read.get(&request.session_id) {
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
