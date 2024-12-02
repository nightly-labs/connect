use crate::{
    state::{SessionToApp, SessionToAppMap, Sessions},
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
    session_to_app: &SessionToAppMap,
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
    let (created_new, app_sessions, session_data) = match sessions_write.get(app_id) {
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
                    let session_data = session_write.client_state.clone();
                    (false, app_sessions.read().await, session_data)
                }
                None => {
                    // Insert a new session into a app sessions map
                    let new_session =
                        Session::new(&session_id, connection_id.clone(), sender, &init_data);
                    let new_session_data = new_session.client_state.clone();

                    app_sessions_write.insert(session_id.clone(), RwLock::new(new_session));

                    // Insert session to app map
                    session_to_app
                        .add_session_to_app(&session_id, &app_id)
                        .await;
                    (true, app_sessions.read().await, new_session_data)
                }
            }
        }
        None => {
            // Creating a new session map and insert session
            let new_session = Session::new(&session_id, connection_id.clone(), sender, &init_data);
            let new_session_data = new_session.client_state.clone();
            let mut app_sessions = HashMap::new();
            app_sessions.insert(session_id.clone(), RwLock::new(new_session));

            sessions_write.insert(app_id.clone(), RwLock::new(app_sessions));

            // Insert session to app map
            session_to_app
                .add_session_to_app(&session_id, &app_id)
                .await;

            let app = match sessions_write.get(app_id) {
                Some(app) => app,
                None => {
                    error!(
                        "App sessions not found after creating a new session; should not happen"
                    );
                    return session_id;
                }
            };
            (true, app.read().await, new_session_data)
        }
    };

    let session = match app_sessions.get(&session_id) {
        Some(session) => session,
        None => {
            error!("Session not found after creating a new session; should not happen");
            return session_id;
        }
    };

    // Prepare the InitializeResponse
    let response = ServerToApp::InitializeResponse(InitializeResponse {
        session_id: session_id.clone(),
        created_new,
        public_keys: session_data.connected_public_keys.clone(),
        response_id: init_data.response_id.clone(),
        metadata: session_data.metadata.clone(),
        app_id: app_id.clone(),
    });

    // Acquire write lock
    let mut session_write = session.write().await;
    // Send the InitializeResponse to the app
    if let Err(err) = session_write.send_to_app(response).await {
        error!("Failed to send InitializeResponse to app: {}", err);
    }

    return session_id;
}
