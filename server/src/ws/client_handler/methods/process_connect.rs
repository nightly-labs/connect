use crate::{
    errors::NightlyError,
    state::{
        ClientSockets, ClientToSessions, ModifySession, SendToClient, SessionToApp,
        SessionToAppMap, Sessions,
    },
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
    session_to_app_map: &SessionToAppMap,
    connect_request: ConnectRequest,
) -> Result<()> {
    let app_id = match session_to_app_map
        .get_app_id(&connect_request.session_id)
        .await
    {
        Some(app_id) => app_id,
        None => {
            let error = ServerToClient::ErrorMessage(ErrorMessage {
                response_id: connect_request.response_id,
                error: NightlyError::UnhandledInternalError.to_string(),
            });
            client_sockets
                .send_to_client(client_id.clone(), error)
                .await
                .unwrap_or_default();

            bail!(
                "Fail, client: {:?} to session: {:?}, app_id not found",
                client_id,
                connect_request.session_id
            );
        }
    };

    let sessions_read = sessions.read().await;
    let app_sessions_read = match sessions_read.get(&app_id) {
        Some(session) => session.read().await,
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
                "Fail, client: {:?}, app_id: {:?}, failed to get sessions for app",
                client_id,
                app_id
            );
        }
    };

    let mut session_write = match app_sessions_read.get(&connect_request.session_id) {
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
                "Fail, client: {:?} to session: {:?}, session does not exist",
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
