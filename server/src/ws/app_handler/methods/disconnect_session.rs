use crate::{
    state::{ClientSockets, ClientToSessions, ModifySession, SendToClient, Sessions},
    structs::{
        client_messages::{
            app_disconnected_event::AppDisconnectedEvent, client_messages::ServerToClient,
        },
        common::SessionStatus,
    },
};
use anyhow::{bail, Result};
use log::warn;
use uuid7::Uuid;

pub async fn disconnect_session(
    app_id: &String,
    session_id: &String,
    connection_id: Uuid,
    sessions: &Sessions,
    client_sockets: &ClientSockets,
    client_to_sessions: &ClientToSessions,
) -> Result<()> {
    // Lock the whole sessions map as we might need to remove a session
    let mut sessions_write = sessions.write().await;
    let mut app_sessions_write = match sessions_write.get_mut(app_id) {
        Some(app_session) => app_session.write().await,
        None => {
            // Should never happen
            bail!("Session not found, session_id: {}", session_id);
        }
    };

    let mut session_write = match app_sessions_write.get_mut(session_id) {
        Some(session) => session.write().await,
        None => {
            // Should never happen
            bail!(
                "Session not found for connection_id: {}, session_id: {}",
                connection_id,
                session_id
            );
        }
    };

    // Close user socket
    if let Some(client_id) = &session_write.client_state.client_id {
        let app_disconnected_event = ServerToClient::AppDisconnectedEvent(AppDisconnectedEvent {
            session_id: session_id.clone(),
            reason: "App disconnected".to_string(),
        });

        if let Err(err) = client_sockets
            .send_to_client(client_id.clone(), app_disconnected_event)
            .await
        {
            warn!(
                "Error sending app disconnected event to client: {}, session_id: {}, err: {}",
                client_id, session_id, err
            );
        }
    }

    // Close app socket
    if let Err(err) = session_write.close_app_socket(&connection_id).await {
        warn!(
            "Error sending app disconnected event to connection_id: {}, session_id: {}, err: {}",
            connection_id, session_id, err
        );
    }

    // Update session status based on session type
    if session_write.persistent {
        session_write.update_status(SessionStatus::AppDisconnected);
    } else {
        // Remove session
        if let Some(client_id) = session_write.client_state.client_id.clone() {
            client_to_sessions
                .remove_session(client_id, session_id.clone())
                .await;
        }

        // Drop session lock
        drop(session_write);

        app_sessions_write.remove(session_id);
    }

    Ok(())
}
