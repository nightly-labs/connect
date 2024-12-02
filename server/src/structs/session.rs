use super::{
    app_messages::{
        app_messages::ServerToApp, initialize::InitializeRequest,
        user_connected_event::UserConnectedEvent, user_disconnected_event::UserDisconnectedEvent,
    },
    common::{AppMetadata, Network, Notification, PendingRequest, SessionStatus, Version},
};
use crate::{state::ClientId, utils::get_timestamp_in_milliseconds};
use anyhow::{bail, Result};
use axum::extract::ws::{Message, WebSocket};
use database::structs::device_metadata::Device;
use futures::{stream::SplitSink, SinkExt};
use log::{info, warn};
use std::collections::HashMap;
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
            info!(
                "Send to app from session: {}, msg: {:?}",
                self.session_id, msg
            );
            let serialized_msg = match serde_json::to_string(&msg) {
                Ok(serialized_msg) => serialized_msg,
                Err(e) => {
                    bail!("Failed to serialize message: {:?}", e);
                }
            };
            socket
                .send(Message::Text(serialized_msg))
                .await
                .unwrap_or_default();
        }

        return Ok(());
    }
    pub async fn close_app_socket(&mut self, id: &Uuid) -> Result<()> {
        info!("Drop app connection for session {}", self.session_id);
        match self.app_state.app_socket.remove(id) {
            Some(mut app_socket) => {
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

    pub async fn connect_user(
        &mut self,
        device: &Option<Device>,
        public_keys: &Vec<String>,
        metadata: &Option<String>,
        client_id: &String,
        notification: &Option<Notification>,
    ) -> Result<()> {
        // Update session status
        self.update_status(SessionStatus::ClientConnected);

        // Update client state
        self.client_state.device = device.clone();
        self.client_state.connected_public_keys = public_keys.clone();
        self.client_state.metadata = metadata.clone();
        self.client_state.client_id = Some(client_id.clone());

        if let Some(notification) = notification {
            self.notification = Some(notification.clone());
        }

        // Send user connected event to app
        let app_event = ServerToApp::UserConnectedEvent(UserConnectedEvent {
            public_keys: public_keys.clone(),
            metadata: metadata.clone(),
        });

        match self.send_to_app(app_event).await {
            Ok(_) => Ok(()),
            Err(err) => bail!("Failed to send message to app: {:?}", err),
        }
    }

    pub async fn disconnect_user(&mut self) {
        // Update session status
        self.update_status(SessionStatus::UserDisconnected);

        // Update client state
        self.client_state = ClientState {
            client_id: None,
            connected_public_keys: vec![],
            device: None,
            metadata: None,
        };
        self.notification = None;
        self.pending_requests.clear();

        // Send disconnect event to app
        let user_disconnected_event = ServerToApp::UserDisconnectedEvent(UserDisconnectedEvent {});
        self.send_to_app(user_disconnected_event)
            .await
            .unwrap_or_default();
    }
}
#[derive(Debug)]
pub struct AppState {
    pub metadata: AppMetadata,
    pub app_socket: HashMap<Uuid, SplitSink<WebSocket, Message>>,
}
#[derive(Debug, Clone)]
pub struct ClientState {
    pub client_id: Option<ClientId>,
    pub device: Option<Device>,
    pub connected_public_keys: Vec<String>,
    pub metadata: Option<String>,
}
