use crate::state::{
    ClientId, ClientToSessions, DisconnectUser, ModifySession, SessionId, SessionToApp,
    SessionToAppMap, Sessions,
};
use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpDropSessionsRequest {
    pub client_id: ClientId,
    pub sessions: Vec<SessionId>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpDropSessionsResponse {
    pub dropped_sessions: Vec<SessionId>,
}

pub async fn drop_sessions(
    State(sessions): State<Sessions>,
    State(client_to_sessions): State<ClientToSessions>,
    State(session_to_app_map): State<SessionToAppMap>,
    Json(request): Json<HttpDropSessionsRequest>,
) -> Result<Json<HttpDropSessionsResponse>, (StatusCode, String)> {
    let mut dropped_sessions = Vec::new();

    for session_id in request.sessions {
        let app_id = match session_to_app_map.get_app_id(&session_id).await {
            Some(app_id) => app_id,
            None => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    "Session does not exist".to_string(),
                ))
            }
        };

        if sessions
            .disconnect_user(session_id.clone(), app_id)
            .await
            .is_ok()
        {
            dropped_sessions.push(session_id.clone());
        };

        client_to_sessions
            .remove_session(request.client_id.clone(), session_id)
            .await;
    }

    Ok(Json(HttpDropSessionsResponse { dropped_sessions }))
}
