use std::{net::SocketAddr, sync::Arc};

use axum::{
    extract::{
        ws::{Message, WebSocket},
        ConnectInfo, State, WebSocketUpgrade,
    },
    response::Response,
};
use dashmap::DashMap;
use futures::{SinkExt, StreamExt};

use crate::structs::{
    app_messages::{
        app_messages::{AppToServer, ServerToApp},
        initialize::InitializeResponse,
    },
    client_messages::{
        app_disconnected_event::AppDisconnectedEvent, client_messages::ServerToClient,
        sign_messages::SignMessagesEvent, sign_transation::SignTransactionsEvent,
    },
    common::SessionStatus,
    pending_request::PendingRequest,
    session::{AppState, ClientState, Session},
};

pub async fn on_new_app_connection(
    ConnectInfo(ip): ConnectInfo<SocketAddr>,
    State(sessions): State<Arc<DashMap<String, Session>>>,
    ws: WebSocketUpgrade,
) -> Response {
    ws.on_upgrade(move |socket| app_handler(socket, sessions))
}

pub async fn app_handler(socket: WebSocket, sessions: Arc<DashMap<String, Session>>) {
    let (sender, mut receiver) = socket.split();
    // Handle the new app connection here
    // Wait for initialize message
    println!("New app connected");
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
                println!("New initialize request");

                let (session_id, created_new) = match init_data.persistent_session_id {
                    Some(session_id) => {
                        let (session_id, created_new) = match sessions.get_mut(session_id.as_str())
                        {
                            Some(mut session) => {
                                session.update_status(SessionStatus::AppConnected);
                                session.app_state = AppState {
                                    metadata: init_data.app_metadata,
                                    app_socket: Some(sender),
                                };
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
                                        app_socket: Some(sender),
                                    },
                                    client_state: ClientState {
                                        client_socket: None,
                                        device: None,
                                        connected_public_keys: Vec::new(),
                                    },
                                    network: init_data.network,
                                    version: init_data.version,
                                    device: None,
                                    pending_requests: DashMap::new(),
                                    notification: None,
                                };
                                sessions.insert(session_id.clone(), session);
                                (session_id.clone(), true)
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
                                app_socket: Some(sender),
                            },
                            client_state: ClientState {
                                client_socket: None,
                                device: None,
                                connected_public_keys: Vec::new(),
                            },
                            network: init_data.network,
                            version: init_data.version,
                            device: None,
                            pending_requests: DashMap::new(),
                            notification: None,
                        };
                        sessions.insert(session_id.clone(), session);
                        (session_id.clone(), true)
                    }
                };

                let mut created_session = sessions.get_mut(&session_id).unwrap();
                // created_session.app_state.app_socket.unwrap().send(item)
                println!("send response");

                match created_session
                    .send_to_app(ServerToApp::InitializeResponse(InitializeResponse {
                        response_id: init_data.response_id,
                        session_id: session_id.clone(),
                        created_new: created_new,
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
                    println!("App disconnected");
                    let app_disconnected_event =
                        ServerToClient::AppDisconnectedEvent(AppDisconnectedEvent {
                            session_id: session_id.clone(),
                            reason: "App disconnected".to_string(),
                        });
                    let mut session = sessions.get_mut(&session_id).unwrap();
                    session
                        .send_to_client(app_disconnected_event)
                        .await
                        .unwrap_or_default();
                    match session.persistent {
                        true => {
                            session.update_status(SessionStatus::AppDisconnected);
                        }
                        false => {
                            match &mut session.client_state.client_socket {
                                Some(client_socket) => {
                                    client_socket.close();
                                }
                                None => {}
                            }
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
                let mut session = sessions.get_mut(&session_id).unwrap();
                session
                    .send_to_client(app_disconnected_event)
                    .await
                    .unwrap_or_default();
                match session.persistent {
                    true => {
                        session.update_status(SessionStatus::AppDisconnected);
                    }
                    false => {
                        match &mut session.client_state.client_socket {
                            Some(client_socket) => {
                                client_socket.close();
                            }
                            None => {}
                        }
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
                let mut session = sessions.get_mut(&session_id).unwrap();
                session
                    .send_to_client(app_disconnected_event)
                    .await
                    .unwrap_or_default();
                match session.persistent {
                    true => {
                        session.update_status(SessionStatus::AppDisconnected);
                    }
                    false => {
                        match &mut session.client_state.client_socket {
                            Some(client_socket) => {
                                client_socket.close();
                            }
                            None => {}
                        }
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
            AppToServer::SignTransactionsRequest(sing_transactions_request) => {
                let mut session = sessions.get_mut(&session_id).unwrap();
                let response_id: String = sing_transactions_request.response_id.clone();
                let pending_request =
                    PendingRequest::SignTransactions(sing_transactions_request.clone());
                session
                    .pending_requests
                    .insert(response_id.clone(), pending_request.clone());
                // Response will be sent by the client side
                let sign_transactions_event =
                    ServerToClient::SignTransactionsEvent(SignTransactionsEvent {
                        request_id: response_id.clone(),
                        transactions: sing_transactions_request.transactions,
                        metadata: sing_transactions_request.metadata,
                    });
                session
                    .send_to_client(sign_transactions_event)
                    .await
                    .unwrap();
            }
            AppToServer::SignMessagesRequest(sign_messages_request) => {
                let mut session = sessions.get_mut(&session_id).unwrap();
                let response_id: String = sign_messages_request.response_id.clone();
                let pending_request = PendingRequest::SignMessages(sign_messages_request.clone());
                session
                    .pending_requests
                    .insert(response_id.clone(), pending_request.clone());
                // Response will be sent by the client side
                let sign_messages_event = ServerToClient::SignMessagesEvent(SignMessagesEvent {
                    request_id: response_id.clone(),
                    messages: sign_messages_request.messages,
                    metadata: sign_messages_request.metadata,
                });
                session.send_to_client(sign_messages_event).await.unwrap();
            }

            AppToServer::InitializeRequest(_) => {
                // App should not send initialize message after the first one
            }
        }
    }
}
