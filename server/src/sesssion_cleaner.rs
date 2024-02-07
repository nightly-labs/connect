use crate::{
    state::{ClientToSessions, ModifySession, Sessions},
    utils::get_timestamp_in_milliseconds,
};
use futures::SinkExt;
use log::info;
use std::{time::Duration, vec};

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

            // Remove all sessions that expired
            let mut sessions_to_remove = vec![];
            let now = get_timestamp_in_milliseconds();

            info!("[{:?}]: Cleaning sessions", now);

            // Lock sessions
            let mut sessions_write = sessions.write().await;

            // Iterate over all sessions and check if they expired
            for (session_id, session) in sessions_write.iter() {
                // Default session time is two weeks
                if session.read().await.creation_timestamp + 1000 * 60 * 60 * 24 * 14 < now {
                    sessions_to_remove.push(session_id.clone());
                }
            }

            info!(
                "[{:?}]: {} sessions to remove",
                now,
                sessions_to_remove.len()
            );

            // Remove all sessions that expired
            for session_id in sessions_to_remove {
                // safe unwrap because we just checked if the session exists
                let session = sessions_write.get_mut(&session_id).unwrap();
                let mut session_write = session.write().await;

                // Remove session from client_to_sessions
                if let Some(client_id) = &session_write.client_state.client_id {
                    client_to_sessions
                        .remove_session(client_id.clone(), session_id.clone())
                        .await;
                }

                // Disconnect app
                // Send to all apps
                for (_, socket) in &mut session_write.app_state.app_socket {
                    socket.close().await.unwrap_or_default();
                }

                // Release write lock on session
                drop(session_write);

                sessions_write.remove(&session_id);
            }

            info!("[{:?}]: Sessions cleaning finished", now);
        }
    });
}
