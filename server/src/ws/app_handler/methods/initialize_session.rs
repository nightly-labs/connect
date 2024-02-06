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
use uuid7::Uuid;

pub async fn initialize_session_connection(
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

    // Lock the whole sessions map as we might need to add a new session
    let mut sessions_lock = sessions.write().await;
    // Check if the session already exists
    let (session_id, created_new) = match sessions_lock.get_mut(&session_id) {
        Some(session) => {
            // Reconnecting to the same persistent session
            session.update_status(SessionStatus::AppConnected);
            session
                .app_state
                .app_socket
                .insert(connection_id.clone(), sender);

            // TODO Additional updates to the session can be done here
            (session_id.clone(), false)
        }
        None => {
            // Creating a new session
            let new_session = Session::new(&session_id, connection_id.clone(), sender, &init_data);
            sessions_lock.insert(session_id.clone(), new_session);
            (session_id, true)
        }
    };

    // At this point, the session is guaranteed to exist, so unwrapping is safe.
    let session = sessions_lock
        .get_mut(&session_id)
        .expect("Session just created or updated; unwrap safe");

    // Send the InitializeResponse to the app
    if let Err(err) = session
        .send_to_app(ServerToApp::InitializeResponse(InitializeResponse {
            session_id: session.session_id.clone(),
            created_new,
            public_keys: session.client_state.connected_public_keys.clone(),
            response_id: init_data.response_id.clone(),
            metadata: session.client_state.metadata.clone(),
        }))
        .await
    {
        error!("Failed to send InitializeResponse to app: {}", err);
    }

    return session_id;
}
