use crate::{
    errors::NightlyError,
    state::{ClientSockets, SendToClient, Sessions},
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
    payload: NewPayloadEventReply,
) -> Result<()> {
    let mut sessions_write = sessions.write().await;
    let session = match sessions_write.get_mut(&payload.session_id) {
        Some(session) => session,
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
    if let None = session.pending_requests.remove(&payload.request_id) {
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
    session.send_to_app(app_msg).await.unwrap_or_default();

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
