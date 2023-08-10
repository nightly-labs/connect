use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{
    errors::NightlyError,
    state::{ClientToSessions, ModifySession, Sessions},
    structs::{
        app_messages::{app_messages::ServerToApp, user_connected_event::UserConnectedEvent},
        common::{Device, Notification, SessionStatus},
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct HttpConnectSessionRequest {
    #[serde(rename = "clientId")]
    pub client_id: String,
    #[serde(rename = "publicKeys")]
    pub public_keys: Vec<String>,
    #[serde(rename = "sessionId")]
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
    let mut sessions = sessions.write().await;
    let mut session = match sessions.get_mut(&request.session_id) {
        Some(session) => session,
        None => {
            return Err((
                StatusCode::BAD_REQUEST,
                NightlyError::SessionDoesNotExist.to_string(),
            ))
        }
    };
    // Insert user socket
    session.update_status(SessionStatus::ClientConnected);
    session.client_state.device = request.device.clone();
    session.client_state.connected_public_keys = request.public_keys.clone();
    session.client_state.metadata = request.metadata.clone();
    session.client_state.client_id = Some(request.client_id.clone());
    // notification
    if let Some(notification) = request.notification.clone() {
        session.notification = Some(notification);
    }
    let app_event = ServerToApp::UserConnectedEvent(UserConnectedEvent {
        public_keys: request.public_keys,
        metadata: request.metadata,
    });
    match session.send_to_app(app_event).await {
        Ok(_) => {}
        Err(_) => {
            return Err((
                StatusCode::BAD_REQUEST,
                NightlyError::AppDisconnected.to_string(),
            ))
        }
    };
    // Insert new session id into client_to_sessions
    client_to_sessions.add_session(request.client_id.clone(), request.session_id.clone());
    return Ok(Json(HttpConnectSessionResponse {}));
}
