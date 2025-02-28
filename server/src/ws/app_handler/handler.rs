use super::methods::{
    disconnect_session::disconnect_session, initialize_session::initialize_session_connection,
};
use crate::{
    state::{
        ClientSockets, ClientToSessions, SendToClient, SessionToApp, SessionToAppMap, Sessions,
    },
    structs::{
        app_messages::app_messages::AppToServer,
        client_messages::{client_messages::ServerToClient, new_payload_event::NewPayloadEvent},
        common::{Device, PendingRequest},
        notification_msg::{trigger_notification, NotificationPayload},
    },
};
use axum::{
    extract::{
        ws::{Message, WebSocket},
        ConnectInfo, State, WebSocketUpgrade,
    },
    response::Response,
};
use futures::StreamExt;
use log::{debug, warn};
use std::net::SocketAddr;

pub async fn on_new_app_connection(
    ConnectInfo(ip): ConnectInfo<SocketAddr>,
    State(sessions): State<Sessions>,
    State(client_sockets): State<ClientSockets>,
    State(client_to_sessions): State<ClientToSessions>,
    State(session_to_app_map): State<SessionToAppMap>,
    ws: WebSocketUpgrade,
) -> Response {
    let ip = ip.clone().to_string().clone();
    ws.on_upgrade(move |socket| async move {
        debug!("OPEN app connection  from {}", ip);
        app_handler(
            socket,
            sessions,
            client_sockets,
            client_to_sessions,
            session_to_app_map,
        )
        .await;
        debug!("CLOSE app connection  from {}", ip);
    })
}

pub async fn app_handler(
    socket: WebSocket,
    sessions: Sessions,
    client_sockets: ClientSockets,
    client_to_sessions: ClientToSessions,
    session_to_app_map: SessionToAppMap,
) {
    let (sender, mut receiver) = socket.split();
    let connection_id = uuid7::uuid7();
    let sessions = sessions.clone();

    // Handle the new app connection here
    // Wait for initialize message
    let (session_id, app_id) = loop {
        // If stream is closed, or message is not received, return
        let msg = match receiver.next().await {
            Some(Ok(msg)) => msg,
            Some(Err(_)) | None => {
                return;
            }
        };

        let app_msg = match msg {
            Message::Text(data) => match serde_json::from_str::<AppToServer>(&data) {
                Ok(app_msg) => app_msg,
                Err(_) => continue,
            },
            Message::Close(None) | Message::Close(Some(_)) => {
                return;
            }
            _ => continue,
        };

        // We only accept initialize messages here
        match app_msg {
            AppToServer::InitializeRequest(init_data) => {
                // TEMP FIX
                let app_id = match &init_data.persistent_session_id {
                    Some(session_id) => session_to_app_map
                        .get_app_id(&session_id)
                        .await
                        .unwrap_or_else(|| {
                            warn!("No app_id found for session: {}", session_id);
                            uuid7::uuid7().to_string()
                        }),
                    None => uuid7::uuid7().to_string(),
                };

                let session_id = initialize_session_connection(
                    &app_id,
                    &connection_id,
                    sender,
                    &sessions,
                    &session_to_app_map,
                    init_data,
                )
                .await;

                break (session_id, app_id);
            }
            _ => {
                continue;
            }
        }
    };

    // Main loop request handler
    loop {
        let msg = match receiver.next().await {
            Some(Ok(msg)) => msg,
            Some(Err(_)) | None => {
                // Disconnect session
                if let Err(err) = disconnect_session(
                    &app_id,
                    &session_id,
                    connection_id,
                    &sessions,
                    &client_sockets,
                    &client_to_sessions,
                    &session_to_app_map,
                )
                .await
                {
                    warn!("Error disconnecting session: {}", err);
                }

                return;
            }
        };

        let app_msg = match msg {
            Message::Text(data) => match serde_json::from_str::<AppToServer>(&data) {
                Ok(app_msg) => app_msg,
                Err(_) => continue,
            },
            Message::Close(None) | Message::Close(Some(_)) => {
                // Disconnect session
                if let Err(err) = disconnect_session(
                    &app_id,
                    &session_id,
                    connection_id,
                    &sessions,
                    &client_sockets,
                    &client_to_sessions,
                    &session_to_app_map,
                )
                .await
                {
                    warn!("Error disconnecting session: {}", err);
                }
                return;
            }
            _ => continue,
        };

        match app_msg {
            AppToServer::RequestPayload(sing_transactions_request) => {
                let sessions_read = sessions.read().await;
                let app_sessions = match sessions_read.get(&app_id) {
                    Some(app_sessions) => app_sessions.read().await,
                    None => {
                        // Should never happen
                        return;
                    }
                };
                let mut session_write = match app_sessions.get(&session_id) {
                    Some(session) => session.write().await,
                    None => {
                        // Should never happen
                        return;
                    }
                };
                let response_id: String = sing_transactions_request.response_id.clone();

                session_write.pending_requests.insert(
                    response_id.clone(),
                    PendingRequest {
                        content: sing_transactions_request.content.clone(),
                        request_id: sing_transactions_request.response_id.clone(),
                    },
                );
                // Response will be sent by the client side
                let sign_transactions_event = ServerToClient::NewPayloadEvent(NewPayloadEvent {
                    payload: sing_transactions_request.content.clone(),
                    request_id: response_id.clone(),
                    session_id: session_id.clone(),
                });

                let client_id = match &session_write.client_state.client_id {
                    Some(id) => id,
                    None => {
                        // Should never happen
                        warn!("No client_id found for session: {}", session_id);
                        continue;
                    }
                };

                // Try to send via WS, this can fail as user might not be subscribed to the session via Ws
                if let Err(_) = client_sockets
                    .send_to_client(client_id.clone(), sign_transactions_event)
                    .await
                {
                    // Fall back to notification
                    if let Some(notification) = &session_write.notification {
                        let notification_payload = NotificationPayload {
                            network: session_write.network.clone(),
                            app_metadata: session_write.app_state.metadata.clone(),
                            device: session_write
                                .client_state
                                .device
                                .clone()
                                .unwrap_or(Device::Unknown),
                            request: sing_transactions_request.content.clone(),
                            request_id: response_id.clone(),
                            session_id: session_id.clone(),
                            token: notification.token.clone(),
                        };

                        // This will never fail as we are not waiting for a response
                        trigger_notification(
                            notification.notification_endpoint.clone(),
                            notification_payload,
                        )
                        .await
                        .unwrap_or_default();
                    }
                }
            }
            AppToServer::DisconnectRequest(_) => {
                // Disconnect session
                if let Err(err) = disconnect_session(
                    &app_id,
                    &session_id,
                    connection_id,
                    &sessions,
                    &client_sockets,
                    &client_to_sessions,
                    &session_to_app_map,
                )
                .await
                {
                    warn!("Error disconnecting session: {}", err);
                }
                return;
            }
            AppToServer::InitializeRequest(_) => {
                // App should not send initialize message after the first one
                continue;
            }
        }
    }
}
