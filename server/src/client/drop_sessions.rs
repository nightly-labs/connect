use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::state::{ClientId, ClientToSessions, ModifySession, SessionId, Sessions};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct HttpDropSessionsRequest {
    #[serde(rename = "clientId")]
    pub client_id: ClientId,
    pub sessions: Vec<SessionId>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct HttpDropSessionsResponse {
    #[serde(rename = "droppedSessions")]
    pub dropped_sessions: Vec<SessionId>,
}
pub async fn drop_sessions(
    State(sessions): State<Sessions>,
    State(client_to_sessions): State<ClientToSessions>,
    Json(request): Json<HttpDropSessionsRequest>,
) -> Result<Json<HttpDropSessionsResponse>, (StatusCode, String)> {
    let mut dropped_sessions = Vec::new();
    // TODO handle disconnecting app
    for session_id in request.sessions {
        if let Some(session) = sessions.get(&session_id) {
            if session.client_state.client_id == Some(request.client_id.clone()) {
                sessions.remove(&session_id);
                client_to_sessions.remove(&session_id);
                dropped_sessions.push(session_id);
            }
        }
    }
    Ok(Json(HttpDropSessionsResponse { dropped_sessions }))
}
