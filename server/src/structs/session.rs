use std::collections::HashMap;

use crate::state::ClientId;

use super::{
    app_messages::app_messages::ServerToApp,
    common::{AppMetadata, Device, Network, Notification, PendingRequest, SessionStatus, Version},
};
use anyhow::Result;
use axum::extract::ws::{Message, WebSocket};
use futures::{stream::SplitSink, SinkExt};
use log::{info, warn};
use uuid7::Uuid;

#[derive(Debug)]
pub struct Session {
    pub session_id: String,
    pub status: SessionStatus,
    pub persistent: bool,
    pub network: Network,
    pub version: Version,
    pub app_state: AppState,
    pub client_state: ClientState,
    pub pending_requests: HashMap<String, PendingRequest>,
    pub notification: Option<Notification>,
    pub creation_timestamp: u64,
}
impl Session {
    pub async fn send_to_app(&mut self, msg: ServerToApp) -> Result<()> {
        // Send to all apps
        for (_, socket) in &mut self.app_state.app_socket {
            info!("Send to app {}, msg: {:?}", self.session_id, msg);
            socket
                .send(Message::Text(
                    serde_json::to_string(&msg).expect("Serialization should work"),
                ))
                .await
                .unwrap_or_default();
        }

        return Ok(());
    }
    pub async fn close_app_socket(&mut self, id: &Uuid) -> Result<()> {
        info!("Drop app connection for session {}", self.session_id);
        match &mut self.app_state.app_socket.remove(id) {
            Some(app_socket) => {
                app_socket.close().await?;
                warn!("Drop app connection for session {}", self.session_id);
                return Ok(());
            }
            None => Err(anyhow::anyhow!("No app socket found for session")),
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
    pub app_socket: HashMap<Uuid, SplitSink<WebSocket, Message>>,
}
#[derive(Debug)]
pub struct ClientState {
    pub client_id: Option<ClientId>,
    pub device: Option<Device>,
    pub connected_public_keys: Vec<String>,
    pub metadata: Option<String>,
}
