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
    let mut sessions = sessions.write().await;

    let session = match sessions.get_mut(&request.session_id) {
        Some(session) => session,
        None => {
            return Err((
                StatusCode::BAD_REQUEST,
                NightlyError::SessionDoesNotExist.to_string(),
            ))
        }
    };
    // Check if client_id matches

    if session.client_state.client_id != Some(request.client_id.clone()) {
        return Err((
            StatusCode::BAD_REQUEST,
            NightlyError::UserNotConnected.to_string(),
        ));
    }
    let _pending_request = match session.pending_requests.remove(&request.request_id) {
        Some(pending_request) => pending_request,
        None => {
            return Err((
                StatusCode::BAD_REQUEST,
                NightlyError::RequestDoesNotExist.to_string(),
            ))
        }
    };
    // Send to app
    let app_msg = ServerToApp::ResponsePayload(ResponsePayload {
        response_id: request.request_id.clone(),
        content: request.content.clone(),
    });
    match session.send_to_app(app_msg).await {
        Ok(_) => {}
        Err(_) => {
            return Err((
                StatusCode::BAD_REQUEST,
                NightlyError::AppDisconnected.to_string(),
            ))
        }
    };
    return Ok(Json(HttpResolveRequestResponse {}));
}
