use std::sync::Arc;

use axum_macros::FromRef;
use dashmap::{DashMap, DashSet};

use crate::structs::session::Session;

pub type SessionId = String;
pub type ClientId = String;
pub type Sessions = Arc<DashMap<SessionId, Session>>;
pub type ClientToSessions = Arc<DashMap<ClientId, DashSet<SessionId>>>;
#[derive(Clone, FromRef)]
pub struct ServerState {
    pub sessions: Sessions,
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
