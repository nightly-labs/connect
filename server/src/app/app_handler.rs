use std::{collections::HashMap, net::SocketAddr};

use axum::{
    extract::{
        ws::{Message, WebSocket},
        ConnectInfo, State, WebSocketUpgrade,
    },
    response::Response,
};
use futures::{SinkExt, StreamExt};
use log::{debug, warn};

use crate::{
    state::{ClientSockets, ClientToSessions, ModifySession, SendToClient, Sessions},
    structs::{
        app_messages::{
            already_connected::AlreadyConnected,
            app_messages::{AppToServer, ServerToApp},
            initialize::InitializeResponse,
        },
        client_messages::{
            app_disconnected_event::AppDisconnectedEvent, client_messages::ServerToClient,
            new_payload_event::NewPayloadEvent,
        },
        common::{Device, PendingRequest, SessionStatus},
        notification_msg::{trigger_notification, NotificationPayload},
        session::{AppState, ClientState, Session},
    },
    utils::get_timestamp_in_milliseconds,
};

pub async fn on_new_app_connection(
    ConnectInfo(ip): ConnectInfo<SocketAddr>,
    State(sessions): State<Sessions>,
    State(client_sockets): State<ClientSockets>,
    State(client_to_sessions): State<ClientToSessions>,
    ws: WebSocketUpgrade,
) -> Response {
    let ip = ip.clone().to_string().clone();
    ws.on_upgrade(move |socket| async move {
        debug!("OPEN app connection  from {}", ip);
        app_handler(socket, sessions, client_sockets, client_to_sessions).await;
        debug!("CLOSE app connection  from {}", ip);
    })
}

pub async fn app_handler(
    socket: WebSocket,
    sessions: Sessions,
    client_sockets: ClientSockets,
    client_to_sessions: ClientToSessions,
) {
    let (mut sender, mut receiver) = socket.split();
    let connection_id = uuid7::uuid7();
    // Handle the new app connection here
    // Wait for initialize message
    let session_id = loop {
        let sessions = sessions.clone();
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
            Message::Text(data) => match serde_json::from_str::<AppToServer>(&data) {
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
            AppToServer::InitializeRequest(init_data) => {
                let (session_id, created_new) = match init_data.persistent_session_id {
                    Some(session_id) => {
                        let (session_id, created_new) = {
                            let mut sessions = sessions.write().await;
                            match sessions.get_mut(session_id.as_str()) {
                                Some(mut session) => {
                                    session.update_status(SessionStatus::AppConnected);
                                    session
                                        .app_state
                                        .app_socket
                                        .insert(connection_id.clone(), sender);
                                    // TODO decide if we want to do anything more here
                                    (session_id.clone(), false)
                                }
                                None => {
                                    let session_id = uuid7::uuid7().to_string();
                                    let session = Session {
                                        session_id: session_id.clone(),
                                        status: SessionStatus::WaitingForClient,
                                        persistent: init_data.persistent,
                                        app_state: AppState {
                                            metadata: init_data.app_metadata,
                                            app_socket: HashMap::from([(
                                                connection_id.clone(),
                                                sender,
                                            )]),
                                        },
                                        client_state: ClientState {
                                            client_id: None,
                                            device: None,
                                            connected_public_keys: Vec::new(),
                                            metadata: None,
                                        },
                                        network: init_data.network,
                                        version: init_data.version,
                                        pending_requests: HashMap::new(),
                                        notification: None,
                                        creation_timestamp: get_timestamp_in_milliseconds(),
                                    };
                                    sessions.insert(session_id.clone(), session);
                                    (session_id.clone(), true)
                                }
                            }
                        };
                        (session_id, created_new)
                    }
                    None => {
                        let session_id = uuid7::uuid7().to_string();
                        let session = Session {
                            session_id: session_id.clone(),
                            status: SessionStatus::WaitingForClient,
                            persistent: init_data.persistent,
                            app_state: AppState {
                                metadata: init_data.app_metadata,
                                app_socket: HashMap::from([(connection_id.clone(), sender)]),
                            },
                            client_state: ClientState {
                                client_id: None,
                                device: None,
                                connected_public_keys: Vec::new(),
                                metadata: None,
                            },
                            network: init_data.network,
                            version: init_data.version,
                            pending_requests: HashMap::new(),
                            notification: None,
                            creation_timestamp: get_timestamp_in_milliseconds(),
                        };
                        sessions.write().await.insert(session_id.clone(), session);
                        (session_id.clone(), true)
                    }
                };

                let mut sessions = sessions.write().await;
                let created_session = sessions.get_mut(&session_id).expect("safe unwrap");
                let public_keys = created_session.client_state.connected_public_keys.clone();
                let metadata = created_session.client_state.metadata.clone();

                match created_session
                    .send_to_app(ServerToApp::InitializeResponse(InitializeResponse {
                        response_id: init_data.response_id,
                        session_id: session_id.clone(),
                        created_new: created_new,
                        public_keys: public_keys,
                        metadata: metadata,
                    }))
                    .await
                {
                    Ok(_) => {}
                    Err(e) => println!("Error sending initialize response: {:?}", e),
                }
                break session_id.clone();
            }
            _ => {
                continue;
            }
        }
    };
    // Main loop request handler
    loop {
        let sessions = sessions.clone();
        let msg = match receiver.next().await {
            Some(msg) => match msg {
                Ok(msg) => msg,
                Err(_e) => {
                    let app_disconnected_event =
                        ServerToClient::AppDisconnectedEvent(AppDisconnectedEvent {
                            session_id: session_id.clone(),
                            reason: "App disconnected".to_string(),
                        });
                    let mut sessions = sessions.write().await;

                    let session = match sessions.get_mut(&session_id) {
                        Some(session) => session,
                        None => {
                            // Should never happen
                            return;
                        }
                    };

                    match &session.client_state.client_id {
                        Some(client_id) => {
                            client_sockets
                                .send_to_client(client_id.clone(), app_disconnected_event)
                                .await
                                .unwrap_or_default();
                        }
                        None => {}
                    }
                    // Close app socket
                    session
                        .close_app_socket(&connection_id)
                        .await
                        .unwrap_or_default();
                    match session.persistent {
                        true => {
                            session.update_status(SessionStatus::AppDisconnected);
                        }
                        false => {
                            // Remove session
                            client_to_sessions.remove_session(
                                session.client_state.client_id.clone().unwrap_or_default(),
                                session_id.clone(),
                            );
                            drop(session);
                            sessions.remove(&session_id);
                        }
                    }
                    return;
                }
            },
            None => {
                let app_disconnected_event =
                    ServerToClient::AppDisconnectedEvent(AppDisconnectedEvent {
                        session_id: session_id.clone(),
                        reason: "App disconnected".to_string(),
                    });
                let mut sessions = sessions.write().await;
                let session = match sessions.get_mut(&session_id) {
                    Some(session) => session,
                    None => {
                        // Should never happen
                        return;
                    }
                };
                match &session.client_state.client_id {
                    Some(client_id) => {
                        client_sockets
                            .send_to_client(client_id.clone(), app_disconnected_event)
                            .await
                            .unwrap_or_default();
                    }
                    None => {}
                }
                // Close app socket
                session
                    .close_app_socket(&connection_id)
                    .await
                    .unwrap_or_default();
                match session.persistent {
                    true => {
                        session.update_status(SessionStatus::AppDisconnected);
                    }
                    false => {
                        // Remove session
                        client_to_sessions.remove_session(
                            session.client_state.client_id.clone().unwrap_or_default(),
                            session_id.clone(),
                        );
                        drop(session);
                        sessions.remove(&session_id);
                    }
                }

                return;
            }
        };
        let app_msg = match msg {
            Message::Text(data) => match serde_json::from_str::<AppToServer>(&data) {
                Ok(app_msg) => app_msg,
                Err(_) => continue,
            },
            Message::Binary(_) => continue,
            Message::Close(None) | Message::Close(Some(_)) => {
                let app_disconnected_event =
                    ServerToClient::AppDisconnectedEvent(AppDisconnectedEvent {
                        session_id: session_id.clone(),
                        reason: "App disconnected".to_string(),
                    });
                let mut sessions = sessions.write().await;
                let session = match sessions.get_mut(&session_id) {
                    Some(session) => session,
                    None => {
                        // Should never happen
                        return;
                    }
                };
                match &session.client_state.client_id {
                    Some(client_id) => {
                        client_sockets
                            .send_to_client(client_id.clone(), app_disconnected_event)
                            .await
                            .unwrap_or_default();
                    }
                    None => {}
                }
                // Close app socket
                session
                    .close_app_socket(&connection_id)
                    .await
                    .unwrap_or_default();
                match session.persistent {
                    true => {
                        session.update_status(SessionStatus::AppDisconnected);
                    }
                    false => {
                        drop(session);
                        sessions.remove(&session_id);
                    }
                }
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
            AppToServer::RequestPayload(sing_transactions_request) => {
                let mut sessions = sessions.write().await;
                let session = match sessions.get_mut(&session_id) {
                    Some(session) => session,
                    None => {
                        // Should never happen
                        return;
                    }
                };
                let response_id: String = sing_transactions_request.response_id.clone();

                session.pending_requests.insert(
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

                let client_id = match &session.client_state.client_id {
                    Some(id) => id,
                    None => {
                        // Should never happen
                        continue;
                    }
                };

                // Try to send via WS
                match client_sockets
                    .send_to_client(client_id.clone(), sign_transactions_event)
                    .await
                {
                    Ok(_) => {}
                    // Fall back to notification
                    Err(_) => {
                        match &session.notification {
                            Some(notification) => {
                                let notification_payload = NotificationPayload {
                                    network: session.network.clone(),
                                    app_metadata: session.app_state.metadata.clone(),
                                    device: session
                                        .client_state
                                        .device
                                        .clone()
                                        .unwrap_or(Device::Unknown),
                                    request: sing_transactions_request.content.clone(),
                                    request_id: response_id.clone(),
                                    session_id: session_id.clone(),
                                    token: notification.token.clone(),
                                };
                                trigger_notification(
                                    notification.notification_endpoint.clone(),
                                    notification_payload,
                                )
                                .await;
                            }
                            None => {
                                // Should we return an error here?
                            }
                        }
                    }
                }
            }

            AppToServer::InitializeRequest(_) => {
                // App should not send initialize message after the first one
            }
        }
    }
}
