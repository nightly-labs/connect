use std::{collections::HashMap, sync::Arc};

use crate::structs::{
    app_messages::{app_messages::ServerToApp, user_disconnected_event::UserDisconnectedEvent},
    client_messages::client_messages::ServerToClient,
    session::{ClientState, Session},
};
use anyhow::Result;
use async_trait::async_trait;
use axum::extract::ws::{Message, WebSocket};
use axum_macros::FromRef;
use dashmap::{DashMap, DashSet};
use futures::{stream::SplitSink, SinkExt};
use log::info;
use tokio::sync::RwLock;

pub type SessionId = String;
pub type ClientId = String;
pub type Sessions = Arc<RwLock<HashMap<SessionId, Session>>>;
pub type ClientSockets = Arc<DashMap<ClientId, SplitSink<WebSocket, Message>>>;
#[async_trait]
pub trait DisconnectUser {
    async fn disconnect_user(&self, session_id: SessionId) -> Result<()>;
}
#[async_trait]
impl DisconnectUser for Sessions {
    async fn disconnect_user(&self, session_id: SessionId) -> Result<()> {
        let mut sessions = self.write().await;
        let mut session = match sessions.get_mut(&session_id) {
            Some(session) => session,
            None => return Err(anyhow::anyhow!("Session does not exist")), // Session does not exist
        };
        session.client_state = ClientState {
            client_id: None,
            connected_public_keys: vec![],
            device: None,
            metadata: None,
        };
        session.notification = None;
        session.pending_requests.clear();
        // Send disconnect event to app
        let user_disconnected_event = ServerToApp::UserDisconnectedEvent(UserDisconnectedEvent {});
        session.send_to_app(user_disconnected_event).await?;
        Ok(())
    }
}
#[async_trait]
pub trait SendToClient {
    async fn send_to_client(&self, client_id: ClientId, msg: ServerToClient) -> Result<()>;
    async fn close_client_socket(&self, client_id: ClientId) -> Result<()>;
}
#[async_trait]
impl SendToClient for ClientSockets {
    async fn send_to_client(&self, client_id: ClientId, msg: ServerToClient) -> Result<()> {
        match &mut self.get_mut(&client_id) {
            Some(client_socket) => {
                info!("Send to client {}, msg: {:?}", client_id, msg);
                return Ok(client_socket
                    .send(Message::Text(
                        serde_json::to_string(&msg).expect("Serialization should work"),
                    ))
                    .await?);
            }
            None => Err(anyhow::anyhow!("No client socket found for session")),
        }
    }
    async fn close_client_socket(&self, client_id: ClientId) -> Result<()> {
        info!("Close client socket {}", client_id);
        match &mut self.remove(&client_id) {
            Some((_, client_socket)) => {
                return Ok(client_socket.close().await?);
            }
            None => Err(anyhow::anyhow!("No client socket found for session")),
        }
    }
}
pub type ClientToSessions = Arc<DashMap<ClientId, DashSet<SessionId>>>;
#[derive(Clone, FromRef)]
pub struct ServerState {
    pub sessions: Sessions,
    pub client_to_sockets: ClientSockets, // Holds only live sockets
    pub client_to_sessions: ClientToSessions,
}
pub trait ModifySession {
    fn remove_session(&self, client_id: ClientId, session_id: SessionId);
    fn add_session(&self, client_id: ClientId, session_id: SessionId);
    fn get_sessions(&self, client_id: ClientId) -> Vec<SessionId>;
}
impl ModifySession for ClientToSessions {
    fn remove_session(&self, client_id: ClientId, session_id: SessionId) {
        let entry = match self.get(&client_id) {
            Some(sessions) => sessions,
            None => return,
        };
        entry.remove(&session_id);
        let is_empty = entry.is_empty();
        drop(entry); // drop the lock
        if is_empty {
            self.remove(&client_id);
        }
    }
    fn add_session(&self, client_id: ClientId, session_id: SessionId) {
        self.entry(client_id)
            .or_insert_with(|| DashSet::new())
            .insert(session_id);
    }
    fn get_sessions(&self, client_id: ClientId) -> Vec<SessionId> {
        match self.get(&client_id) {
            Some(sessions) => sessions.iter().map(|session| session.clone()).collect(),
            None => vec![],
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modify_session() {
        // Create a new ClientToSessions instance for testing
        let client_to_sessions = ClientToSessions::default();

        // Add a session
        let client_id = "client1".to_string();
        let session_id = "session1".to_string();
        client_to_sessions.add_session(client_id.clone(), session_id.clone());

        // Get sessions for the client
        let sessions = client_to_sessions.get_sessions(client_id.clone());
        assert_eq!(sessions, vec![session_id.clone()]);

        // Remove the session
        client_to_sessions.remove_session(client_id.clone(), session_id.clone());

        // Ensure the session is removed
        let sessions = client_to_sessions.get_sessions(client_id.clone());
        assert!(sessions.is_empty());

        // Ensure the client is removed
        let maybe_sessions = client_to_sessions.get(&client_id);
        assert!(maybe_sessions.is_none());
    }
}
