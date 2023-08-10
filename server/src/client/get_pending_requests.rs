use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{
    errors::NightlyError,
    state::{ClientId, SessionId, Sessions},
    structs::common::PendingRequest,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct HttpGetPendingRequestsRequest {
    #[serde(rename = "clientId")]
    pub client_id: ClientId,
    #[serde(rename = "sessionId")]
    pub session_id: SessionId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct HttpGetPendingRequestsResponse {
    #[serde(rename = "pendingRequests")]
    pub pending_requests: Vec<PendingRequest>,
}
pub async fn get_pending_requests(
    State(sessions): State<Sessions>,
    Json(request): Json<HttpGetPendingRequestsRequest>,
) -> Result<Json<HttpGetPendingRequestsResponse>, (StatusCode, String)> {
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
    if session.client_state.client_id != Some(request.client_id.clone()) {
        return Err((
            StatusCode::BAD_REQUEST,
            NightlyError::UserNotConnected.to_string(),
        ));
    }
    let mut pending_requests = Vec::new();
    for (key, pending_request) in session.pending_requests.iter() {
        pending_requests.push(pending_request.clone());
    }
    Ok(Json(HttpGetPendingRequestsResponse { pending_requests }))
}
