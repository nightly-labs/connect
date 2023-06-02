use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{
    state::{ClientId, Sessions},
    structs::{
        app_messages::{app_messages::ServerToApp, sign_messages::SignMessagesResponse},
        common::SignedMessage,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ResolveSignMessageRequestRequest {
    #[serde(rename = "clientId")]
    pub client_id: ClientId,
    #[serde(rename = "requestId")]
    pub request_id: String,
    #[serde(rename = "signedMessages")]
    pub signed_messages: Vec<SignedMessage>,
    #[ts(optional)]
    pub metadata: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ResolveSignMessagesRequestResponse {}
pub async fn resolve_sign_messages_request(
    State(sessions): State<Sessions>,
    Json(request): Json<ResolveSignMessageRequestRequest>,
) -> Result<Json<ResolveSignMessagesRequestResponse>, (StatusCode, String)> {
    let mut session = match sessions.get_mut(&request.request_id) {
        Some(session) => session,
        None => {
            return Err((
                StatusCode::BAD_REQUEST,
                "Invalid request or session not found".to_string(),
            ))
        }
    };
    // Check if client_id matches

    if session.client_state.client_id != Some(request.client_id.clone()) {
        return Err((
            StatusCode::BAD_REQUEST,
            "Invalid request or session not found".to_string(),
        ));
    }
    let _pending_request = match session.pending_requests.remove(&request.request_id) {
        Some(pending_request) => pending_request,
        None => {
            return Err((
                StatusCode::BAD_REQUEST,
                "Invalid request or session not found".to_string(),
            ))
        }
    };
    // Send to app
    let app_msg = ServerToApp::SignMessagesResponse(SignMessagesResponse {
        response_id: request.request_id.clone(),
        signed_messages: request.signed_messages,
        metadata: request.metadata,
    });
    session.send_to_app(app_msg).await.unwrap();
    return Ok(Json(ResolveSignMessagesRequestResponse {}));
}
