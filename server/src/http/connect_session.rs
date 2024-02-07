use crate::{
    errors::NightlyError,
    state::{ClientToSessions, ModifySession, Sessions},
    structs::common::{Device, Notification},
};
use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpConnectSessionRequest {
    pub client_id: String,
    pub public_keys: Vec<String>,
    pub session_id: String,
    #[ts(optional)]
    pub notification: Option<Notification>,
    #[ts(optional)]
    pub device: Option<Device>,
    #[ts(optional)]
    pub metadata: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct HttpConnectSessionResponse {}
pub async fn connect_session(
    State(sessions): State<Sessions>,
    State(client_to_sessions): State<ClientToSessions>,
    Json(request): Json<HttpConnectSessionRequest>,
) -> Result<Json<HttpConnectSessionResponse>, (StatusCode, String)> {
    let sessions_read = sessions.read().await;
    let mut session_write = match sessions_read.get(&request.session_id) {
        Some(session) => session.write().await,
        None => {
            return Err((
                StatusCode::BAD_REQUEST,
                NightlyError::SessionDoesNotExist.to_string(),
            ))
        }
    };

    // Insert user socket
    session_write
        .connect_user(
            &request.device,
            &request.public_keys,
            &request.metadata,
            &request.client_id,
            &request.notification,
        )
        .await
        .map_err(|_| {
            return (
                StatusCode::BAD_REQUEST,
                NightlyError::AppDisconnected.to_string(),
            );
        })?;

    // Insert new session id into client_to_sessions
    client_to_sessions
        .add_session(request.client_id.clone(), request.session_id.clone())
        .await;

    return Ok(Json(HttpConnectSessionResponse {}));
}
