use std::{time::Duration, vec};

use futures::SinkExt;

use crate::{
    state::{ClientToSessions, ModifySession, Sessions},
    utils::get_timestamp_in_milliseconds,
};

pub fn start_cleaning_sessions(sessions: Sessions, client_to_sessions: ClientToSessions) {
    let sessions = sessions.clone();
    let client_to_sessions = client_to_sessions.clone();
    tokio::spawn(async move {
        // Run this once every 6 hours
        let mut interval: tokio::time::Interval =
            tokio::time::interval(Duration::from_secs(60 * 60 * 6));
        loop {
            // Wait for tick
            interval.tick().await;
            let mut sessions_to_remove = vec![];
            // Remove all sessions that expired
            let now = get_timestamp_in_milliseconds();
            let mut sessions = sessions.write().await;
            for (session_id, session) in sessions.iter() {
                // Check if the session expired
                // Default session time is two weeks
                if session.creation_timestamp + 1000 * 60 * 60 * 24 * 14 < now {
                    sessions_to_remove.push(session_id.clone());
                }
            }
            // Remove all sessions that expired
            for session_id in sessions_to_remove {
                let session = sessions.get_mut(&session_id).unwrap();
                // Remove session from client_to_sessions
                match &session.client_state.client_id {
                    Some(client_id) => {
                        client_to_sessions.remove_session(client_id.clone(), session_id.clone());
                    }
                    None => {}
                }
                // Disconnect app
                // Send to all apps
                for (_, socket) in &mut session.app_state.app_socket {
                    let _ = socket.close();
                }

                drop(session);
                sessions.remove(&session_id);
            }
        }
    });
}
