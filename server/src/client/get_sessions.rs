use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::state::{ClientId, ClientToSessions, ModifySession, SessionId};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct GetSessionsRequest {
    #[serde(rename = "clientId")]
    client_id: ClientId,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct GetSessionsResponse {
    sessions: Vec<SessionId>,
}
pub async fn get_sessions(
    State(client_to_sessions): State<ClientToSessions>,
    Json(request): Json<GetSessionsRequest>,
) -> Result<Json<GetSessionsResponse>, (StatusCode, String)> {
    let sessions = client_to_sessions.get_sessions(request.client_id);
    Ok(Json(GetSessionsResponse { sessions }))
}
