use crate::{
    errors::NightlyError,
    state::{ClientSockets, SendToClient, Sessions},
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
    get_info_request: GetInfoRequest,
) -> Result<()> {
    let mut sessions_write = sessions.write().await;
    let session = match sessions_write.get_mut(&get_info_request.session_id) {
        Some(session) => session,
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
        network: session.network.clone(),
        version: session.version.clone(),
        app_metadata: session.app_state.metadata.clone(),
    });
    client_sockets
        .send_to_client(client_id.clone(), response)
        .await
        .unwrap_or_default();

    Ok(())
}
