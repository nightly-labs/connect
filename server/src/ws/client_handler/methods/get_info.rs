use crate::{
    errors::NightlyError,
    state::{ClientSockets, SendToClient, SessionToApp, SessionToAppMap, Sessions},
    structs::{
        client_messages::{
            client_messages::ServerToClient,
            get_info::{GetInfoRequest, GetInfoResponse},
        },
        common::ErrorMessage,
    },
};
use anyhow::{bail, Result};

pub async fn process_get_info(
    client_id: &String,
    sessions: &Sessions,
    client_sockets: &ClientSockets,
    session_to_app_map: &SessionToAppMap,
    get_info_request: GetInfoRequest,
) -> Result<()> {
    let app_id = match session_to_app_map
        .get_app_id(&get_info_request.session_id)
        .await
    {
        Some(app_id) => app_id,
        None => {
            let error = ServerToClient::ErrorMessage(ErrorMessage {
                response_id: get_info_request.response_id,
                error: NightlyError::UnhandledInternalError.to_string(),
            });
            client_sockets
                .send_to_client(client_id.clone(), error)
                .await
                .unwrap_or_default();

            bail!(
                "Fail, client: {:?} to session: {:?}, app_id not found",
                client_id,
                get_info_request.session_id
            );
        }
    };

    let sessions_read = sessions.read().await;
    let app_sessions_read = match sessions_read.get(&app_id) {
        Some(session) => session.read().await,
        None => {
            let error = ServerToClient::ErrorMessage(ErrorMessage {
                response_id: get_info_request.response_id,
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

    let session_read = match app_sessions_read.get(&get_info_request.session_id) {
        Some(session) => session.read().await,
        None => {
            let error = ServerToClient::ErrorMessage(ErrorMessage {
                response_id: get_info_request.response_id,
                error: NightlyError::SessionDoesNotExist.to_string(),
            });
            client_sockets
                .send_to_client(client_id.clone(), error)
                .await
                .unwrap_or_default();

            bail!(
                "Fail, client: {:?} to session: {:?}, session does not exist",
                client_id,
                get_info_request.session_id
            );
        }
    };

    let response = ServerToClient::GetInfoResponse(GetInfoResponse {
        response_id: get_info_request.response_id,
        network: session_read.network.clone(),
        version: session_read.version.clone(),
        app_metadata: session_read.app_state.metadata.clone(),
    });
    client_sockets
        .send_to_client(client_id.clone(), response)
        .await
        .unwrap_or_default();

    Ok(())
}
