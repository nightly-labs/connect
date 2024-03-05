use crate::{
    errors::NightlyError,
    state::{ClientId, SessionToApp, SessionToAppMap, Sessions},
    structs::app_messages::{app_messages::ServerToApp, payload::ResponsePayload},
};
use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpResolveRequestRequest {
    pub client_id: ClientId,
    pub session_id: String,
    pub request_id: String,
    pub content: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct HttpResolveRequestResponse {}

pub async fn resolve_request(
    State(sessions): State<Sessions>,
    State(session_to_app_map): State<SessionToAppMap>,
    Json(request): Json<HttpResolveRequestRequest>,
) -> Result<Json<HttpResolveRequestResponse>, (StatusCode, String)> {
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

    let mut session_write = match app_sessions_read.get(&request.session_id) {
        Some(session) => session.write().await,
        None => {
            return Err((
                StatusCode::BAD_REQUEST,
                NightlyError::SessionDoesNotExist.to_string(),
            ))
        }
    };

    // Check if client_id matches
    if session_write.client_state.client_id != Some(request.client_id.clone()) {
        return Err((
            StatusCode::BAD_REQUEST,
            NightlyError::UserNotConnected.to_string(),
        ));
    }
    // Remove request from pending requests
    if let None = session_write.pending_requests.remove(&request.request_id) {
        return Err((
            StatusCode::BAD_REQUEST,
            NightlyError::RequestDoesNotExist.to_string(),
        ));
    };

    // Send to app
    let app_msg = ServerToApp::ResponsePayload(ResponsePayload {
        response_id: request.request_id.clone(),
        content: request.content.clone(),
    });
    if let Err(_) = session_write.send_to_app(app_msg).await {
        return Err((
            StatusCode::BAD_REQUEST,
            NightlyError::AppDisconnected.to_string(),
        ));
    };

    return Ok(Json(HttpResolveRequestResponse {}));
}
