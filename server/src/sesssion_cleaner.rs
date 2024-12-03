use crate::{
    state::{ClientToSessions, ModifySession, SessionToApp, SessionToAppMap, Sessions},
    utils::get_timestamp_in_milliseconds,
};
use futures::SinkExt;
use log::{error, info};
use std::{collections::HashMap, time::Duration, vec};

pub fn start_cleaning_sessions(
    sessions: &Sessions,
    client_to_sessions: &ClientToSessions,
    session_to_app_map: &SessionToAppMap,
) {
    let sessions = sessions.clone();
    let client_to_sessions = client_to_sessions.clone();
    let session_to_app_map = session_to_app_map.clone();

    tokio::spawn(async move {
        // Run this once every 6 hours
        let mut interval: tokio::time::Interval =
            tokio::time::interval(Duration::from_secs(60 * 60 * 6));
        loop {
            // Wait for tick
            interval.tick().await;

            // Remove all sessions that expired
            let mut sessions_to_remove: HashMap<String, Vec<String>> = HashMap::new();
            let now = get_timestamp_in_milliseconds();

            info!("[{:?}]: Cleaning sessions", now);

            // Lock sessions
            let mut sessions_write = sessions.write().await;

            // Iterate over all sessions and check if they expired
            for (app_id, sessions) in sessions_write.iter() {
                for (session_id, session) in sessions.read().await.iter() {
                    // If client has not connected to the session within a hour, remove the session
                    if session.read().await.creation_timestamp + 1000 * 60 * 60 < now {
                        sessions_to_remove
                            .entry(app_id.clone())
                            .or_default()
                            .push(session_id.clone());

                        continue;
                    }

                    // Default session time is one week
                    if session.read().await.creation_timestamp + 1000 * 60 * 60 * 24 * 7 < now {
                        // Check if the session is still in use
                        let session = session.read().await;
                        // If user is still connected, don't remove the session
                        if session.client_state.client_id.is_some() {
                            continue;
                        }

                        // If app is still connected, don't remove the session as well
                        if !session.app_state.app_socket.is_empty() {
                            continue;
                        }

                        sessions_to_remove
                            .entry(app_id.clone())
                            .or_default()
                            .push(session_id.clone());
                    }
                }
            }

            info!(
                "[{:?}]: {} sessions to remove",
                now,
                sessions_to_remove.len()
            );

            // save the app entries to remove
            let mut app_entries_to_remove = vec![];

            // Remove all sessions that expired
            for (app_id, session_id) in sessions_to_remove {
                let app_sessions = match sessions_write.get_mut(&app_id) {
                    Some(app_sessions) => app_sessions,
                    None => {
                        error!("App: [{}] does not have any sessions", app_id);
                        return;
                    }
                };
                let mut app_sessions_write = app_sessions.write().await;

                for session_id in session_id {
                    let session = match app_sessions_write.get_mut(&session_id) {
                        Some(session) => session,
                        None => {
                            error!(
                                "App: [{}] does not have session with id: [{}]",
                                app_id, session_id
                            );
                            return;
                        }
                    };
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

                    app_sessions_write.remove(&session_id);

                    // Remove session from session_to_app_map
                    session_to_app_map
                        .remove_session_from_app(&session_id)
                        .await;
                }

                if app_sessions_write.is_empty() {
                    app_entries_to_remove.push(app_id.clone());
                }
            }

            info!(
                "[{:?}]: {} empty app entries to remove",
                now,
                app_entries_to_remove.len()
            );

            // Clear empty app entries
            for app_id in app_entries_to_remove {
                sessions_write.remove(&app_id);
            }

            info!("[{:?}]: Sessions cleaning finished", now);
        }
    });
}
