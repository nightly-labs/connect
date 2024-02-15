use crate::{
    state::Sessions,
    structs::{
        app_messages::{
            app_messages::ServerToApp,
            initialize::{InitializeRequest, InitializeResponse},
        },
        common::SessionStatus,
        session::Session,
    },
};
use axum::extract::ws::{Message, WebSocket};
use futures::stream::SplitSink;
use log::error;
use std::collections::HashMap;
use tokio::sync::RwLock;
use uuid7::Uuid;

pub async fn initialize_session_connection(
    app_id: &String,
    connection_id: &Uuid,
    sender: SplitSink<WebSocket, Message>,
    sessions: &Sessions,
    init_data: InitializeRequest,
) -> String {
    // If the session_id is not provided, generate a new one
    let session_id = init_data
        .persistent_session_id
        .clone()
        .unwrap_or_else(|| uuid7::uuid7().to_string());

    // Lock the whole sessions map as we might need to add a new app sessions entry
    let mut sessions_write = sessions.write().await;
    // Check if the app sessions already exists
    let created_new = match sessions_write.get(app_id) {
        Some(app_sessions) => {
            let mut app_sessions_write = app_sessions.write().await;

            match app_sessions_write.get_mut(&session_id) {
                Some(session) => {
                    // Reconnecting to the same persistent session
                    let mut session_write = session.write().await;
                    session_write.update_status(SessionStatus::AppConnected);
                    session_write
                        .app_state
                        .app_socket
                        .insert(connection_id.clone(), sender);

                    false
                }
                None => {
                    // Insert a new session into a app sessions map
                    let new_session =
                        Session::new(&session_id, connection_id.clone(), sender, &init_data);
                    app_sessions_write.insert(session_id.clone(), RwLock::new(new_session));
                    true
                }
            }
        }
        None => {
            // Creating a new session map and insert session
            let new_session = Session::new(&session_id, connection_id.clone(), sender, &init_data);
            let mut app_sessions = HashMap::new();
            app_sessions.insert(session_id.clone(), RwLock::new(new_session));

            sessions_write.insert(app_id.clone(), RwLock::new(app_sessions));
            true
        }
    };

    let app_sessions_read = sessions_write
        .get(&session_id)
        .expect("Session just created or updated; unwrap safe")
        .read()
        .await;

    let session = app_sessions_read
        .get(&session_id)
        .expect("Session just created or updated; unwrap safe");

    // Prepare the InitializeResponse
    let session_read = session.read().await;
    let response = ServerToApp::InitializeResponse(InitializeResponse {
        session_id: session_id.clone(),
        created_new,
        public_keys: session_read.client_state.connected_public_keys.clone(),
        response_id: init_data.response_id.clone(),
        metadata: session_read.client_state.metadata.clone(),
    });
    // Drop session read lock
    drop(session_read);

    // Acquire write lock
    let mut session_write = session.write().await;
    // Send the InitializeResponse to the app
    if let Err(err) = session_write.send_to_app(response).await {
        error!("Failed to send InitializeResponse to app: {}", err);
    }

    return session_id;
}
