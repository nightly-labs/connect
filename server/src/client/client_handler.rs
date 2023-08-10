use std::net::SocketAddr;

use axum::{
    extract::{
        ws::{Message, WebSocket},
        ConnectInfo, State, WebSocketUpgrade,
    },
    response::Response,
};
use futures::StreamExt;
use log::{debug, info};

use crate::{
    errors::NightlyError,
    state::{
        ClientSockets, ClientToSessions, DisconnectUser, ModifySession, SendToClient, Sessions,
    },
    structs::{
        app_messages::{
            app_messages::ServerToApp, payload::ResponsePayload,
            user_connected_event::UserConnectedEvent,
            user_disconnected_event::UserDisconnectedEvent,
        },
        client_messages::{
            client_initialize::ClientInitializeResponse,
            client_messages::{ClientToServer, ServerToClient},
            connect::ConnectResponse,
            drop_sessions::{self, DropSessionsResponse},
            get_info::GetInfoResponse,
            get_pending_requests::GetPendingRequestsResponse,
            get_sessions::GetSessionsResponse,
        },
        common::{AckMessage, ErrorMessage, SessionStatus},
    },
};

pub async fn on_new_client_connection(
    ConnectInfo(ip): ConnectInfo<SocketAddr>,
    State(sessions): State<Sessions>,
    State(client_sockets): State<ClientSockets>,
    State(client_to_sessions): State<ClientToSessions>,
    ws: WebSocketUpgrade,
) -> Response {
    let ip = ip.clone().to_string().clone();
    ws.on_upgrade(move |socket| async move {
        debug!("OPEN client connection  from {}", ip);
        client_handler(socket, sessions, client_sockets, client_to_sessions).await;
        debug!("CLOSE client connection from {}", ip);
    })
}

pub async fn client_handler(
    socket: WebSocket,
    sessions: Sessions,
    client_sockets: ClientSockets,
    client_to_sessions: ClientToSessions,
) {
    let (sender, mut receiver) = socket.split();
    // Handle the new app connection here
    // Wait for initialize message
    let client_id: String = loop {
        let msg = match receiver.next().await {
            Some(msg) => match msg {
                Ok(msg) => msg,
                Err(_e) => {
                    return;
                }
            },
            None => {
                return;
            }
        };
        let app_msg = match msg {
            Message::Text(data) => match serde_json::from_str::<ClientToServer>(&data) {
                Ok(app_msg) => app_msg,
                Err(_) => continue,
            },
            Message::Binary(_) => continue,
            Message::Close(None) | Message::Close(Some(_)) => {
                return;
            }
            Message::Ping(_) => {
                continue;
            }
            Message::Pong(_) => {
                continue;
            }
        };
        match app_msg {
            ClientToServer::ClientInitializeRequest(connect_request) => {
                client_sockets.insert(connect_request.client_id.clone(), sender);
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
        let sessions = sessions.clone();
        let msg = match receiver.next().await {
            Some(msg) => match msg {
                Ok(msg) => msg,
                Err(_e) => {
                    let user_disconnected_event =
                        ServerToApp::UserDisconnectedEvent(UserDisconnectedEvent {});
                    let user_sessions = client_to_sessions.get_sessions(client_id.clone());
                    for session_id in user_sessions {
                        let mut sessions = sessions.write().await;
                        let session = match sessions.get_mut(&session_id) {
                            Some(session) => session,
                            None => {
                                // Should never happen
                                continue;
                            }
                        };
                        session
                            .send_to_app(user_disconnected_event.clone())
                            .await
                            .unwrap_or_default();
                        session.update_status(SessionStatus::UserDisconnected);
                    }
                    // Remove client socket
                    client_sockets
                        .close_client_socket(client_id.clone())
                        .await
                        .unwrap_or_default();
                    return;
                }
            },
            None => {
                let user_disconnected_event =
                    ServerToApp::UserDisconnectedEvent(UserDisconnectedEvent {});
                let user_sessions = client_to_sessions.get_sessions(client_id.clone());
                for session_id in user_sessions {
                    let mut sessions = sessions.write().await;
                    let session = match sessions.get_mut(&session_id) {
                        Some(session) => session,
                        None => {
                            continue;
                        }
                    };
                    session
                        .send_to_app(user_disconnected_event.clone())
                        .await
                        .unwrap_or_default();
                    session.update_status(SessionStatus::UserDisconnected);
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
            Message::Binary(_) => continue,
            Message::Close(None) | Message::Close(Some(_)) => {
                let user_disconnected_event =
                    ServerToApp::UserDisconnectedEvent(UserDisconnectedEvent {});
                let user_sessions = client_to_sessions.get_sessions(client_id.clone());
                for session_id in user_sessions {
                    let mut sessions = sessions.write().await;
                    let session = match sessions.get_mut(&session_id) {
                        Some(session) => session,
                        None => {
                            continue;
                        }
                    };
                    session
                        .send_to_app(user_disconnected_event.clone())
                        .await
                        .unwrap_or_default();
                    session.update_status(SessionStatus::UserDisconnected);
                }
                // Remove client socket
                client_sockets
                    .close_client_socket(client_id.clone())
                    .await
                    .unwrap_or_default();
                return;
            }
            Message::Ping(_) => {
                continue;
            }
            Message::Pong(_) => {
                continue;
            }
        };
        info!("Client {} new msg {:?}", client_id, app_msg);

        match app_msg {
            ClientToServer::ConnectRequest(connect_request) => {
                let mut sessions = sessions.write().await;
                let session = match sessions.get_mut(&connect_request.session_id) {
                    Some(session) => session,
                    None => {
                        let error = ServerToClient::ErrorMessage(ErrorMessage {
                            response_id: connect_request.response_id,
                            error: NightlyError::SessionDoesNotExist.to_string(),
                        });
                        client_sockets
                            .send_to_client(client_id.clone(), error)
                            .await
                            .unwrap_or_default();

                        info!(
                            "Client {} session does not exist {}",
                            client_id, connect_request.session_id
                        );

                        continue;
                    }
                };

                // Insert user socket
                session.update_status(SessionStatus::ClientConnected);
                session.client_state.device = connect_request.device.clone();
                session.client_state.connected_public_keys = connect_request.public_keys.clone();
                session.client_state.metadata = connect_request.metadata.clone();
                session.client_state.client_id = Some(connect_request.client_id.clone());
                // Setup notification
                match &connect_request.notification {
                    Some(notification) => {
                        session.notification = Some(notification.clone());
                    }
                    None => {
                        // skip
                    }
                }
                let app_event = ServerToApp::UserConnectedEvent(UserConnectedEvent {
                    public_keys: connect_request.public_keys,
                    metadata: connect_request.metadata,
                });
                session.send_to_app(app_event).await.unwrap_or_default();

                // Insert new session id into client_to_sessions
                client_to_sessions.add_session(
                    connect_request.client_id.clone(),
                    connect_request.session_id.clone(),
                );

                let client_reponse = ServerToClient::ConnectResponse(ConnectResponse {
                    response_id: connect_request.response_id,
                });
                client_sockets
                    .send_to_client(client_id.clone(), client_reponse)
                    .await
                    .unwrap_or_default();
            }
            ClientToServer::NewPayloadEventReply(new_payload_event_reply) => {
                let mut sessions = sessions.write().await;
                let session = match sessions.get_mut(&new_payload_event_reply.session_id) {
                    Some(session) => session,
                    None => {
                        let error = ServerToClient::ErrorMessage(ErrorMessage {
                            response_id: new_payload_event_reply.response_id,
                            error: NightlyError::SessionDoesNotExist.to_string(),
                        });
                        client_sockets
                            .send_to_client(client_id.clone(), error)
                            .await
                            .unwrap_or_default();
                        info!(
                            "Client {} session does not exist {}",
                            client_id, new_payload_event_reply.session_id
                        );
                        continue;
                    }
                };
                match session
                    .pending_requests
                    .remove(&new_payload_event_reply.request_id)
                {
                    Some(_) => {}
                    None => {
                        let error = ServerToClient::ErrorMessage(ErrorMessage {
                            response_id: new_payload_event_reply.response_id,
                            error: NightlyError::RequestDoesNotExist.to_string(),
                        });
                        client_sockets
                            .send_to_client(client_id.clone(), error)
                            .await
                            .unwrap_or_default();
                        continue;
                    }
                };
                // Send to app
                let app_msg = ServerToApp::ResponsePayload(ResponsePayload {
                    response_id: new_payload_event_reply.request_id.clone(),
                    content: new_payload_event_reply.content.clone(),
                });
                session.send_to_app(app_msg).await.unwrap_or_default();

                let client_msg = ServerToClient::AckMessage(AckMessage {
                    response_id: new_payload_event_reply.response_id,
                });
                client_sockets
                    .send_to_client(client_id.clone(), client_msg)
                    .await
                    .unwrap_or_default();
            }
            ClientToServer::GetInfoRequest(get_info_request) => {
                let sessions = sessions.read().await;
                let session = match sessions.get(&get_info_request.session_id) {
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
                        info!(
                            "Client {} session does not exist {}",
                            client_id, get_info_request.session_id
                        );
                        continue;
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
            }
            ClientToServer::GetPendingRequestsRequest(get_pending_requests_request) => {
                let sessions = sessions.read().await;
                let session = match sessions.get(&get_pending_requests_request.session_id) {
                    Some(session) => session,
                    None => {
                        let error = ServerToClient::ErrorMessage(ErrorMessage {
                            response_id: get_pending_requests_request.response_id,
                            error: NightlyError::SessionDoesNotExist.to_string(),
                        });
                        client_sockets
                            .send_to_client(client_id.clone(), error)
                            .await
                            .unwrap_or_default();
                        info!(
                            "Client {} session does not exist {}",
                            client_id, get_pending_requests_request.session_id
                        );
                        continue;
                    }
                };
                let pending_requests = session
                    .pending_requests
                    .clone()
                    .iter()
                    .map(|(_, v)| v.clone())
                    .collect::<Vec<_>>();
                let response =
                    ServerToClient::GetPendingRequestsResponse(GetPendingRequestsResponse {
                        requests: pending_requests,
                        response_id: get_pending_requests_request.response_id,
                    });
                client_sockets
                    .send_to_client(client_id.clone(), response)
                    .await
                    .unwrap_or_default();
            }
            ClientToServer::GetSessionsRequest(get_sessions_request) => {
                let sessions = client_to_sessions.get_sessions(client_id.clone());
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
                let mut dropped_sessions = Vec::new();
                // TODO handle disconnecting app
                for session_id in drop_sessions_request.sessions {
                    if sessions.disconnect_user(session_id.clone()).await.is_ok() {
                        dropped_sessions.push(session_id.clone());
                    };
                    if let Some(sessions) = client_to_sessions.get_mut(&client_id) {
                        sessions.remove(&session_id);
                    }
                }
                let response = ServerToClient::DropSessionsResponse(DropSessionsResponse {
                    dropped_sessions,
                    response_id: drop_sessions_request.response_id,
                });
                client_sockets
                    .send_to_client(client_id.clone(), response)
                    .await
                    .unwrap_or_default();
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
