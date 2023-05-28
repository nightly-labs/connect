use super::{
    app_messages::app_messages::ServerToApp,
    common::{Device, Network, SessionStatus, Version},
    pending_request::PendingRequest,
};
use anyhow::Result;
use axum::extract::ws::{Message, WebSocket};
use dashmap::DashMap;
use futures::{stream::SplitSink, SinkExt};

#[derive(Debug)]
pub struct Session {
    pub id: String,
    pub status: SessionStatus,
    pub persistent: bool,
    pub network: Network,
    pub version: Version,
    pub device: Option<Device>,
    pub app_state: AppState,
    pub client_state: ClientState,
    pub pending_requests: DashMap<String, PendingRequest>,
    pub token: Option<String>,
    pub notification_endpoint: Option<String>,
    pub connected_public_keys: Vec<String>,
}
impl Session {
    pub async fn send_app_response(&mut self, app_response: ServerToApp) -> Result<()> {
        match &mut self.app_state.app_socket {
            Some(app_socket) => Ok(app_socket
                .send(Message::Text(serde_json::to_string(&app_response).unwrap()))
                .await
                .unwrap()),
            None => Err(anyhow::anyhow!("No app socket found for session")),
        }
    }
}
#[derive(Debug)]
pub struct AppState {
    pub app_name: String,
    pub app_description: Option<String>,
    pub app_icon: Option<String>,
    pub additional_info: Option<String>,
    pub app_socket: Option<SplitSink<WebSocket, Message>>,
}
#[derive(Debug)]
pub struct ClientState {
    pub device: Option<Device>,
    pub client_socket: Option<SplitSink<WebSocket, Message>>,
}
