use crate::{
    errors::NightlyError,
    state::{ClientSockets, SendToClient, SessionToApp, SessionToAppMap, Sessions},
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
    session_to_app_map: &SessionToAppMap,
    get_pending_requests: GetPendingRequestsRequest,
) -> Result<()> {
    let app_id = match session_to_app_map
        .get_app_id(&get_pending_requests.session_id)
        .await
    {
        Some(app_id) => app_id,
        None => {
            let error = ServerToClient::ErrorMessage(ErrorMessage {
                response_id: get_pending_requests.response_id,
                error: NightlyError::UnhandledInternalError.to_string(),
            });
            client_sockets
                .send_to_client(client_id.clone(), error)
                .await
                .unwrap_or_default();

            bail!(
                "Fail, client: {:?} to session: {:?}, app_id not found",
                client_id,
                get_pending_requests.session_id
            );
        }
    };

    let sessions_read = sessions.read().await;
    let app_sessions_read = match sessions_read.get(&app_id) {
        Some(session) => session.read().await,
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
                "Fail, client: {:?}, app_id: {:?}, failed to get sessions for app",
                client_id,
                app_id
            );
        }
    };

    let session_read = match app_sessions_read.get(&get_pending_requests.session_id) {
        Some(session) => session.read().await,
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

    let pending_requests = session_read
        .pending_requests
        .values()
        .cloned()
        .collect::<Vec<_>>();

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
