use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{
    state::{ClientId, SessionId, Sessions},
    structs::pending_request::PendingRequest,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct GetPendingRequestsRequest {
    #[serde(rename = "clientId")]
    pub client_id: ClientId,
    #[serde(rename = "sessionId")]
    pub session_id: SessionId,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct GetPendingRequestsResponse {
    #[serde(rename = "pendingRequests")]
    pub pending_requests: Vec<PendingRequest>,
}
pub async fn get_pending_requests(
    State(sessions): State<Sessions>,
    Json(request): Json<GetPendingRequestsRequest>,
) -> Result<Json<GetPendingRequestsResponse>, (StatusCode, String)> {
    let mut pending_requests = Vec::new();
    if let Some(session) = sessions.get(&request.session_id) {
        if session.client_state.client_id == Some(request.client_id.clone()) {
            for pending_request in session.pending_requests.iter() {
                pending_requests.push(pending_request.clone());
            }
        }
    }
    Ok(Json(GetPendingRequestsResponse { pending_requests }))
}
