use crate::{
    ip_geolocation::GeolocationRequester,
    structs::{
        client_messages::client_messages::ServerToClient, session::Session,
        wallet_metadata::WalletMetadata,
    },
};
use anyhow::Result;
use async_trait::async_trait;
use axum::extract::{
    ws::{Message, WebSocket},
    FromRef,
};
use database::db::Db;
use futures::{stream::SplitSink, SinkExt};
use log::info;
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};
use tokio::sync::RwLock;

pub type SessionId = String;
pub type ClientId = String;
pub type AppId = String;
pub type Sessions = Arc<RwLock<HashMap<AppId, RwLock<HashMap<SessionId, RwLock<Session>>>>>>;
pub type ClientSockets = Arc<RwLock<HashMap<ClientId, RwLock<SplitSink<WebSocket, Message>>>>>;
pub type ClientToSessions = Arc<RwLock<HashMap<ClientId, RwLock<HashSet<SessionId>>>>>;

#[derive(Clone, FromRef)]
pub struct ServerState {
    pub sessions: Sessions,
    pub client_to_sockets: ClientSockets, // Holds only live sockets
    pub client_to_sessions: ClientToSessions,
    pub wallets_metadata: Arc<Vec<WalletMetadata>>,
    pub session_to_app_map: SessionToAppMap,
    pub db: Option<Arc<Db>>,
    pub geo_location: Option<Arc<GeolocationRequester>>,
}

#[async_trait]
pub trait DisconnectUser {
    async fn disconnect_user(&self, session_id: SessionId, app_id: AppId) -> Result<()>;
}
#[async_trait]
impl DisconnectUser for Sessions {
    async fn disconnect_user(&self, session_id: SessionId, app_id: AppId) -> Result<()> {
        match self.read().await.get(&app_id) {
            Some(app) => match app.read().await.get(&session_id) {
                Some(session) => {
                    // Update session user state
                    session.write().await.disconnect_user().await;
                }
                None => {
                    return Err(anyhow::anyhow!(
                        "This app does not have session with provided id"
                    ))
                }
            },
            None => return Err(anyhow::anyhow!("App does not have any sessions")),
        }

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
        match self.read().await.get(&client_id) {
            Some(client_socket) => {
                info!("Send to client {}, msg: {:?}", client_id, msg);
                return Ok(client_socket
                    .write()
                    .await
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
        match self.write().await.remove(&client_id) {
            Some(client_socket) => {
                return Ok(client_socket.write().await.close().await?);
            }
            None => Err(anyhow::anyhow!("No client socket found for session")),
        }
    }
}

#[async_trait]
pub trait ModifySession {
    async fn remove_session(&self, client_id: ClientId, session_id: SessionId);
    async fn add_session(&self, client_id: ClientId, session_id: SessionId);
    async fn get_sessions(&self, client_id: ClientId) -> Vec<SessionId>;
}

#[async_trait]
impl ModifySession for ClientToSessions {
    async fn remove_session(&self, client_id: ClientId, session_id: SessionId) {
        let mut clients_write = self.write().await;

        let client_sessions_lock = match clients_write.get_mut(&client_id) {
            Some(entry) => entry,
            None => return,
        };

        let mut client_sessions_write = client_sessions_lock.write().await;
        client_sessions_write.remove(&session_id);

        let is_empty = client_sessions_write.is_empty();
        drop(client_sessions_write);

        if is_empty {
            clients_write.remove(&client_id);
        }
    }

    async fn add_session(&self, client_id: ClientId, session_id: SessionId) {
        let mut clients_write = self.write().await;
        let client_sessions = clients_write
            .entry(client_id)
            .or_insert_with(|| RwLock::new(HashSet::new()));

        client_sessions.write().await.insert(session_id);
    }

    async fn get_sessions(&self, client_id: ClientId) -> Vec<SessionId> {
        match self.read().await.get(&client_id) {
            Some(sessions) => sessions.read().await.iter().cloned().collect(),
            None => vec![],
        }
    }
}

pub type SessionToAppMap = Arc<RwLock<HashMap<SessionId, AppId>>>;

#[async_trait]
pub trait SessionToApp {
    async fn add_session_to_app(&self, session_id: &SessionId, app_id: &AppId);
    async fn remove_session_from_app(&self, session_id: &SessionId);
    async fn get_app_id(&self, session_id: &SessionId) -> Option<AppId>;
}

#[async_trait]
impl SessionToApp for SessionToAppMap {
    async fn add_session_to_app(&self, session_id: &SessionId, app_id: &AppId) {
        let mut session_to_app_write = self.write().await;
        session_to_app_write.insert(session_id.clone(), app_id.clone());
    }

    async fn remove_session_from_app(&self, session_id: &SessionId) {
        let mut session_to_app_write = self.write().await;
        session_to_app_write.remove(session_id);
    }

    async fn get_app_id(&self, session_id: &SessionId) -> Option<AppId> {
        self.read().await.get(session_id).cloned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_modify_session() {
        // Create a new ClientToSessions instance for testing
        let client_to_sessions = ClientToSessions::default();

        // Add a session
        let client_id = "client1".to_string();
        let session_id = "session1".to_string();
        client_to_sessions
            .add_session(client_id.clone(), session_id.clone())
            .await;

        // Get sessions for the client
        let sessions = client_to_sessions.get_sessions(client_id.clone()).await;
        assert_eq!(sessions, vec![session_id.clone()]);

        // Remove the session
        client_to_sessions
            .remove_session(client_id.clone(), session_id.clone())
            .await;

        // Ensure the session is removed
        let sessions = client_to_sessions.get_sessions(client_id.clone()).await;
        assert!(sessions.is_empty());

        // Ensure the client is removed
        let client_to_sessions_read = client_to_sessions.read().await;
        let maybe_sessions = client_to_sessions_read.get(&client_id);
        assert!(maybe_sessions.is_none());
    }
}
