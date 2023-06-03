use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::state::{ClientId, ClientToSessions, ModifySession, SessionId};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct HttpGetSessionsRequest {
    #[serde(rename = "clientId")]
    client_id: ClientId,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct HttpGetSessionsResponse {
    sessions: Vec<SessionId>,
}
pub async fn get_sessions(
    State(client_to_sessions): State<ClientToSessions>,
    Json(request): Json<HttpGetSessionsRequest>,
) -> Result<Json<HttpGetSessionsResponse>, (StatusCode, String)> {
    let sessions = client_to_sessions.get_sessions(request.client_id);
    Ok(Json(HttpGetSessionsResponse { sessions }))
}
