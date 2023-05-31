use super::{
    app_messages::app_messages::ServerToApp,
    client_messages::client_messages::ServerToClient,
    common::{AppMetadata, Device, Network, Notification, SessionStatus, Version},
    pending_request::PendingRequest,
};
use anyhow::Result;
use axum::extract::ws::{Message, WebSocket};
use dashmap::DashMap;
use futures::{stream::SplitSink, SinkExt};

#[derive(Debug)]
pub struct Session {
    pub session_id: String,
    pub status: SessionStatus,
    pub persistent: bool,
    pub network: Network,
    pub version: Version,
    pub device: Option<Device>,
    pub app_state: AppState,
    pub client_state: ClientState,
    pub pending_requests: DashMap<String, PendingRequest>,
    pub notification: Option<Notification>,
}
impl Session {
    pub async fn send_to_app(&mut self, msg: ServerToApp) -> Result<()> {
        match &mut self.app_state.app_socket {
            Some(app_socket) => Ok(app_socket
                .send(Message::Text(serde_json::to_string(&msg).unwrap()))
                .await?),
            None => Err(anyhow::anyhow!("No app socket found for session")),
        }
    }
    pub async fn send_to_client(&mut self, msg: ServerToClient) -> Result<()> {
        match &mut self.client_state.client_socket {
            Some(client_socket) => Ok(client_socket
                .send(Message::Text(serde_json::to_string(&msg).unwrap()))
                .await?),
            None => Err(anyhow::anyhow!("No client socket found for session")),
        }
    }
    pub fn update_status(&mut self, status: SessionStatus) {
        match status {
            SessionStatus::ClientConnected => {
                self.status = status;
            }
            SessionStatus::AppConnected => {
                self.status = status;
            }
            SessionStatus::UserDisconnected => {
                if self.status == SessionStatus::AppDisconnected {
                    self.status = SessionStatus::Idle;
                } else {
                    self.status = status;
                }
            }
            SessionStatus::AppDisconnected => {
                if self.status == SessionStatus::UserDisconnected {
                    self.status = SessionStatus::Idle;
                } else {
                    self.status = status;
                }
            }
            SessionStatus::Idle => {
                self.status = status;
            }
            SessionStatus::WaitingForClient => {
                self.status = status;
            }
        }
    }
}
#[derive(Debug)]
pub struct AppState {
    pub metadata: AppMetadata,
    pub app_socket: Option<SplitSink<WebSocket, Message>>,
}
#[derive(Debug)]
pub struct ClientState {
    pub device: Option<Device>,
    pub client_socket: Option<SplitSink<WebSocket, Message>>,
    pub connected_public_keys: Vec<String>,
}
