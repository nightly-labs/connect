use std::collections::HashMap;

use crate::{state::ClientId, utils::get_timestamp_in_milliseconds};

use super::{
    app_messages::{
        app_messages::ServerToApp, initialize::InitializeRequest,
        user_connected_event::UserConnectedEvent,
    },
    client_messages::connect::ConnectRequest,
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

    pub fn new(
        session_id: &str,
        connection_id: Uuid,
        sender: SplitSink<WebSocket, Message>,
        init_data: &InitializeRequest,
    ) -> Self {
        Session {
            session_id: session_id.to_owned(),
            status: SessionStatus::WaitingForClient,
            persistent: init_data.persistent,
            app_state: AppState {
                metadata: init_data.app_metadata.clone(),
                app_socket: HashMap::from([(connection_id, sender)]),
            },
            client_state: ClientState {
                client_id: None,
                device: None,
                connected_public_keys: Vec::new(),
                metadata: None,
            },
            network: init_data.network.clone(),
            version: init_data.version.clone(),
            pending_requests: HashMap::new(),
            notification: None,
            creation_timestamp: get_timestamp_in_milliseconds(),
        }
    }

    pub async fn connect_user(&mut self, connect_request: &ConnectRequest) {
        // Update session status
        self.update_status(SessionStatus::ClientConnected);

        // Update client state
        self.client_state.device = connect_request.device.clone();
        self.client_state.connected_public_keys = connect_request.public_keys.clone();
        self.client_state.metadata = connect_request.metadata.clone();
        self.client_state.client_id = Some(connect_request.client_id.clone());

        if let Some(notification) = &connect_request.notification {
            self.notification = Some(notification.clone());
        }

        // Send user connected event to app
        let app_event = ServerToApp::UserConnectedEvent(UserConnectedEvent {
            public_keys: connect_request.public_keys.clone(),
            metadata: connect_request.metadata.clone(),
        });
        self.send_to_app(app_event).await.unwrap_or_default();
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
