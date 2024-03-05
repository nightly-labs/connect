use crate::{
    errors::NightlyError,
    state::{ClientId, SessionId, SessionToApp, SessionToAppMap, Sessions},
    structs::common::PendingRequest,
};
use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpGetPendingRequestRequest {
    pub client_id: ClientId,
    pub session_id: SessionId,
    pub request_id: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct HttpGetPendingRequestResponse {
    pub request: PendingRequest,
}
pub async fn get_pending_request(
    State(sessions): State<Sessions>,
    State(session_to_app_map): State<SessionToAppMap>,
    Json(request): Json<HttpGetPendingRequestRequest>,
) -> Result<Json<HttpGetPendingRequestResponse>, (StatusCode, String)> {
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

    if session_read.client_state.client_id != Some(request.client_id.clone()) {
        return Err((
            StatusCode::BAD_REQUEST,
            NightlyError::UserNotConnected.to_string(),
        ));
    }

    match session_read.pending_requests.get(&request.request_id) {
        Some(pending_request) => {
            return Ok(Json(HttpGetPendingRequestResponse {
                request: pending_request.clone(),
            }))
        }
        None => {
            return Err((
                StatusCode::BAD_REQUEST,
                NightlyError::RequestDoesNotExist.to_string(),
            ))
        }
    };
}
