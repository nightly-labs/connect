use crate::{
    errors::NightlyError,
    state::{ClientSockets, SendToClient, Sessions},
    structs::{
        client_messages::{
            client_messages::ServerToClient,
            get_pending_requests::{GetPendingRequestsRequest, GetPendingRequestsResponse},
        },
        common::ErrorMessage,
    },
};
use anyhow::{bail, Result};

pub async fn process_get_pending_requests(
    client_id: &String,
    sessions: &Sessions,
    client_sockets: &ClientSockets,
    get_pending_requests: GetPendingRequestsRequest,
) -> Result<()> {
    let pending_requests = match sessions.read().await.get(&get_pending_requests.session_id) {
        Some(session) => session
            .read()
            .await
            .pending_requests
            .values()
            .cloned()
            .collect::<Vec<_>>(),
        None => {
            let error = ServerToClient::ErrorMessage(ErrorMessage {
                response_id: get_pending_requests.response_id,
                error: NightlyError::SessionDoesNotExist.to_string(),
            });
            client_sockets
                .send_to_client(client_id.clone(), error)
                .await
                .unwrap_or_default();

            bail!(
                "Fail, client: {:?} to session: {:?}, session does not exist",
                client_id,
                get_pending_requests.session_id
            );
        }
    };

    let response = ServerToClient::GetPendingRequestsResponse(GetPendingRequestsResponse {
        requests: pending_requests,
        response_id: get_pending_requests.response_id,
    });
    client_sockets
        .send_to_client(client_id.clone(), response)
        .await
        .unwrap_or_default();

    Ok(())
}
