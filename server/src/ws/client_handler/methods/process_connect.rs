use crate::{
    errors::NightlyError,
    state::{ClientSockets, ClientToSessions, ModifySession, SendToClient, Sessions},
    structs::{
        client_messages::{
            client_messages::ServerToClient,
            connect::{ConnectRequest, ConnectResponse},
        },
        common::ErrorMessage,
    },
};
use anyhow::{bail, Result};

pub async fn process_client_connect(
    client_id: &String,
    sessions: &Sessions,
    client_sockets: &ClientSockets,
    client_to_sessions: &ClientToSessions,
    connect_request: ConnectRequest,
) -> Result<()> {
    let sessions_read = sessions.read().await;
    let mut session_write = match sessions_read.get(&connect_request.session_id) {
        Some(session) => session.write().await,
        None => {
            let error = ServerToClient::ErrorMessage(ErrorMessage {
                response_id: connect_request.response_id,
                error: NightlyError::SessionDoesNotExist.to_string(),
            });
            client_sockets
                .send_to_client(client_id.clone(), error)
                .await
                .unwrap_or_default();

            bail!(
                "Failed to connect client: {:?} to session: {:?}, session does not exist",
                client_id,
                connect_request.session_id
            );
        }
    };

    // Update session
    session_write
        .connect_user(
            &connect_request.device,
            &connect_request.public_keys,
            &connect_request.metadata,
            &connect_request.client_id,
            &connect_request.notification,
        )
        .await
        .unwrap_or_default();

    // Insert new session id into client_to_sessions
    client_to_sessions
        .add_session(
            connect_request.client_id.clone(),
            connect_request.session_id.clone(),
        )
        .await;

    let client_response = ServerToClient::ConnectResponse(ConnectResponse {
        response_id: connect_request.response_id,
    });
    client_sockets
        .send_to_client(client_id.clone(), client_response)
        .await
        .unwrap_or_default();

    Ok(())
}
