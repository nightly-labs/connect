use crate::state::{ClientToSessions, DisconnectUser, ModifySession, Sessions};
use anyhow::{bail, Result};

pub async fn disconnect_client_sessions(
    client_id: String,
    sessions: &Sessions,
    client_to_sessions: &ClientToSessions,
    sessions_list: Option<&Vec<String>>,
) -> Result<Vec<String>> {
    // If not specified get all sessions for the client
    let user_sessions = match sessions_list {
        Some(sessions) => sessions.clone(),
        None => client_to_sessions.get_sessions(client_id.clone()).await,
    };

    let mut dropped_sessions = Vec::new();

    // Send user disconnected event to all sessions
    for session_id in user_sessions {
        if sessions.disconnect_user(session_id.clone()).await.is_ok() {
            dropped_sessions.push(session_id.clone());
        };

        client_to_sessions
            .remove_session(client_id.clone(), session_id.clone())
            .await;
    }

    if dropped_sessions.is_empty() {
        bail!("No sessions found for client")
    } else {
        Ok(dropped_sessions)
    }
}
