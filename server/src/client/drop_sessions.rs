use crate::state::{
    ClientId, ClientToSessions, DisconnectUser, ModifySession, SessionId, Sessions,
};
use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

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
        if sessions.disconnect_user(session_id.clone()).await.is_ok() {
            dropped_sessions.push(session_id.clone());
        };

        client_to_sessions
            .remove_session(request.client_id.clone(), session_id)
            .await;
    }
    Ok(Json(HttpDropSessionsResponse { dropped_sessions }))
}
