use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{
    errors::NightlyError,
    state::{ClientId, Sessions},
    structs::app_messages::{app_messages::ServerToApp, payload::ResponsePayload},
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct HttpResolveRequestRequest {
    #[serde(rename = "clientId")]
    pub client_id: ClientId,
    #[serde(rename = "sessionId")]
    pub session_id: String,
    #[serde(rename = "requestId")]
    pub request_id: String,
    pub content: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct HttpResolveRequestResponse {}
pub async fn resolve_request(
    State(sessions): State<Sessions>,
    Json(request): Json<HttpResolveRequestRequest>,
) -> Result<Json<HttpResolveRequestResponse>, (StatusCode, String)> {
    // Get session
    let sessions_read = sessions.read().await;
    let mut session_write = match sessions_read.get(&request.session_id) {
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
