use crate::{
    errors::NightlyError,
    state::{
        ClientSockets, ClientToSessions, ModifySession, SendToClient, SessionToAppMap, Sessions,
    },
    structs::{
        client_messages::{
            client_initialize::ClientInitializeResponse,
            client_messages::{ClientToServer, ServerToClient},
            drop_sessions::DropSessionsResponse,
            get_sessions::GetSessionsResponse,
        },
        common::ErrorMessage,
    },
    ws::client_handler::methods::{
        disconnect_sessions::disconnect_client_sessions, get_info::process_get_info,
        get_pending_requests::process_get_pending_requests,
        process_connect::process_client_connect,
        process_new_payload_reply::process_new_payload_event_reply,
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
use log::{debug, info, warn};
use std::net::SocketAddr;
use tokio::sync::RwLock;

pub async fn on_new_client_connection(
    ConnectInfo(ip): ConnectInfo<SocketAddr>,
    State(sessions): State<Sessions>,
    State(client_sockets): State<ClientSockets>,
    State(client_to_sessions): State<ClientToSessions>,
    State(session_to_app_map): State<SessionToAppMap>,
    ws: WebSocketUpgrade,
) -> Response {
    let ip = ip.clone().to_string().clone();
    ws.on_upgrade(move |socket| async move {
        debug!("OPEN client connection  from {}", ip);
        client_handler(
            socket,
            sessions,
            client_sockets,
            client_to_sessions,
            session_to_app_map,
        )
        .await;
        debug!("CLOSE client connection from {}", ip);
    })
}

pub async fn client_handler(
    socket: WebSocket,
    sessions: Sessions,
    client_sockets: ClientSockets,
    client_to_sessions: ClientToSessions,
    session_to_app_map: SessionToAppMap,
) {
    let (sender, mut receiver) = socket.split();
    let sessions = sessions.clone();
    // Handle the new app connection here
    // Wait for initialize message
    let client_id: String = loop {
        // If stream is closed, or message is not received, return
        let msg = match receiver.next().await {
            Some(Ok(msg)) => msg,
            Some(Err(_)) | None => {
                return;
            }
        };

        let app_msg = match msg {
            Message::Text(data) => match serde_json::from_str::<ClientToServer>(&data) {
                Ok(app_msg) => app_msg,
                Err(_) => continue,
            },
            Message::Close(None) | Message::Close(Some(_)) => {
                return;
            }
            _ => continue,
        };

        match app_msg {
            ClientToServer::ClientInitializeRequest(connect_request) => {
                // Insert client socket
                {
                    let mut client_sockets_write = client_sockets.write().await;
                    client_sockets_write
                        .insert(connect_request.client_id.clone(), RwLock::new(sender));
                }
                // Send response
                let client_msg =
                    ServerToClient::ClientInitializeResponse(ClientInitializeResponse {
                        response_id: connect_request.response_id,
                    });
                client_sockets
                    .send_to_client(connect_request.client_id.clone(), client_msg)
                    .await
                    .unwrap_or_default();
                break connect_request.client_id;
            }
            _ => {
                continue;
            }
        }
    };
    info!("Client connected: {}", client_id);

    // Main loop request handler
    loop {
        let msg = match receiver.next().await {
            Some(Ok(msg)) => msg,
            Some(Err(_)) | None => {
                // Disconnect all user sessions
                if let Err(err) = disconnect_client_sessions(
                    client_id.clone(),
                    &sessions,
                    &client_to_sessions,
                    &session_to_app_map,
                    None,
                )
                .await
                {
                    warn!("Error disconnecting session: {}", err);
                }

                // Remove client socket
                client_sockets
                    .close_client_socket(client_id.clone())
                    .await
                    .unwrap_or_default();

                return;
            }
        };

        let app_msg = match msg {
            Message::Text(data) => match serde_json::from_str::<ClientToServer>(&data) {
                Ok(app_msg) => app_msg,
                Err(_) => continue,
            },
            Message::Close(None) | Message::Close(Some(_)) => {
                // Disconnect all user sessions
                if let Err(err) = disconnect_client_sessions(
                    client_id.clone(),
                    &sessions,
                    &client_to_sessions,
                    &session_to_app_map,
                    None,
                )
                .await
                {
                    warn!("Error disconnecting session: {}", err);
                }
                // Remove client socket
                client_sockets
                    .close_client_socket(client_id.clone())
                    .await
                    .unwrap_or_default();

                return;
            }
            _ => continue,
        };
        info!("Client {} new msg {:?}", client_id, app_msg);

        match app_msg {
            ClientToServer::ConnectRequest(connect_request) => {
                if let Err(err) = process_client_connect(
                    &client_id,
                    &sessions,
                    &client_sockets,
                    &client_to_sessions,
                    &session_to_app_map,
                    connect_request,
                )
                .await
                {
                    warn!("Error processing connect request: {}", err);
                }
            }
            ClientToServer::NewPayloadEventReply(new_payload_event_reply) => {
                if let Err(err) = process_new_payload_event_reply(
                    &client_id,
                    &sessions,
                    &client_sockets,
                    &session_to_app_map,
                    new_payload_event_reply,
                )
                .await
                {
                    warn!("Error processing new payload event reply: {}", err);
                }
            }
            ClientToServer::GetInfoRequest(get_info_request) => {
                if let Err(err) = process_get_info(
                    &client_id,
                    &sessions,
                    &client_sockets,
                    &session_to_app_map,
                    get_info_request,
                )
                .await
                {
                    warn!("Error processing get info request: {}", err);
                }
            }
            ClientToServer::GetPendingRequestsRequest(get_pending_requests_request) => {
                if let Err(err) = process_get_pending_requests(
                    &client_id,
                    &sessions,
                    &client_sockets,
                    &session_to_app_map,
                    get_pending_requests_request,
                )
                .await
                {
                    warn!("Error processing get pending requests request: {}", err);
                }
            }
            ClientToServer::GetSessionsRequest(get_sessions_request) => {
                let sessions = client_to_sessions.get_sessions(client_id.clone()).await;
                let response = ServerToClient::GetSessionsResponse(GetSessionsResponse {
                    sessions,
                    response_id: get_sessions_request.response_id,
                });
                client_sockets
                    .send_to_client(client_id.clone(), response)
                    .await
                    .unwrap_or_default();
            }
            ClientToServer::DropSessionsRequest(drop_sessions_request) => {
                match disconnect_client_sessions(
                    client_id.clone(),
                    &sessions,
                    &client_to_sessions,
                    &session_to_app_map,
                    Some(&drop_sessions_request.sessions),
                )
                .await
                {
                    Ok(dropped_sessions) => {
                        let response = ServerToClient::DropSessionsResponse(DropSessionsResponse {
                            dropped_sessions,
                            response_id: drop_sessions_request.response_id,
                        });
                        client_sockets
                            .send_to_client(client_id.clone(), response)
                            .await
                            .unwrap_or_default();
                    }
                    Err(err) => {
                        let error = ServerToClient::ErrorMessage(ErrorMessage {
                            response_id: drop_sessions_request.response_id,
                            error: err.to_string(),
                        });
                        client_sockets
                            .send_to_client(client_id.clone(), error)
                            .await
                            .unwrap_or_default();
                    }
                }
            }
            ClientToServer::ClientInitializeRequest(_) => {
                let error = ServerToClient::ErrorMessage(ErrorMessage {
                    response_id: "".to_string(),
                    error: NightlyError::ClientAlreadyInitialized.to_string(),
                });
                client_sockets
                    .send_to_client(client_id.clone(), error)
                    .await
                    .unwrap_or_default();
            }
        }
        info!("Client {} msg handled", client_id);
    }
}
