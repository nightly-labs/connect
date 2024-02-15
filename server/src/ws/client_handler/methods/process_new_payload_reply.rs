use crate::{
    errors::NightlyError,
    state::{ClientSockets, SendToClient, SessionToApp, SessionToAppMap, Sessions},
    structs::{
        app_messages::{app_messages::ServerToApp, payload::ResponsePayload},
        client_messages::{
            client_messages::ServerToClient, new_payload_event::NewPayloadEventReply,
        },
        common::{AckMessage, ErrorMessage},
    },
};
use anyhow::{bail, Result};

pub async fn process_new_payload_event_reply(
    client_id: &String,
    sessions: &Sessions,
    client_sockets: &ClientSockets,
    session_to_app_map: &SessionToAppMap,
    payload: NewPayloadEventReply,
) -> Result<()> {
    let app_id = match session_to_app_map.get_app_id(&payload.session_id).await {
        Some(app_id) => app_id,
        None => {
            let error = ServerToClient::ErrorMessage(ErrorMessage {
                response_id: payload.response_id,
                error: NightlyError::UnhandledInternalError.to_string(),
            });
            client_sockets
                .send_to_client(client_id.clone(), error)
                .await
                .unwrap_or_default();

            bail!(
                "Fail, client: {:?} to session: {:?}, app_id not found",
                client_id,
                payload.session_id
            );
        }
    };

    let sessions_read = sessions.read().await;
    let app_sessions_read = match sessions_read.get(&app_id) {
        Some(session) => session.read().await,
        None => {
            let error = ServerToClient::ErrorMessage(ErrorMessage {
                response_id: payload.response_id,
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

    let mut session_write = match app_sessions_read.get(&payload.session_id) {
        Some(session) => session.write().await,
        None => {
            let error = ServerToClient::ErrorMessage(ErrorMessage {
                response_id: payload.response_id,
                error: NightlyError::SessionDoesNotExist.to_string(),
            });
            client_sockets
                .send_to_client(client_id.clone(), error)
                .await
                .unwrap_or_default();

            bail!(
                "Fail, client: {:?} to session: {:?}, session does not exist",
                client_id,
                payload.session_id
            );
        }
    };

    // Update session
    if let None = session_write.pending_requests.remove(&payload.request_id) {
        let error: ServerToClient = ServerToClient::ErrorMessage(ErrorMessage {
            response_id: payload.response_id,
            error: NightlyError::RequestDoesNotExist.to_string(),
        });
        client_sockets
            .send_to_client(client_id.clone(), error)
            .await
            .unwrap_or_default();

        bail!("Failed to process new payload event reply, request does not exist, session_id: {:?}, request_id: {:?}", payload.session_id, payload.request_id);
    };

    // Send response to app
    let app_msg = ServerToApp::ResponsePayload(ResponsePayload {
        response_id: payload.request_id.clone(),
        content: payload.content.clone(),
    });
    session_write.send_to_app(app_msg).await.unwrap_or_default();

    // Send ack to client
    let client_msg = ServerToClient::AckMessage(AckMessage {
        response_id: payload.response_id,
    });
    client_sockets
        .send_to_client(client_id.clone(), client_msg)
        .await
        .unwrap_or_default();

    Ok(())
}
