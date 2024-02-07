use crate::{
    errors::NightlyError,
    state::{ClientId, SessionId, Sessions},
    structs::common::PendingRequest,
};
use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpGetPendingRequestsRequest {
    pub client_id: ClientId,
    pub session_id: SessionId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpGetPendingRequestsResponse {
    pub pending_requests: Vec<PendingRequest>,
}
pub async fn get_pending_requests(
    State(sessions): State<Sessions>,
    Json(request): Json<HttpGetPendingRequestsRequest>,
) -> Result<Json<HttpGetPendingRequestsResponse>, (StatusCode, String)> {
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

    if session_read.client_state.client_id != Some(request.client_id.clone()) {
        return Err((
            StatusCode::BAD_REQUEST,
            NightlyError::UserNotConnected.to_string(),
        ));
    }

    let pending_requests = session_read
        .pending_requests
        .values()
        .cloned()
        .collect::<Vec<_>>();

    Ok(Json(HttpGetPendingRequestsResponse { pending_requests }))
}
