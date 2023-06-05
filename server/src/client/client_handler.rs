use std::net::SocketAddr;

use axum::{
    extract::{
        ws::{Message, WebSocket},
        ConnectInfo, State, WebSocketUpgrade,
    },
    response::Response,
};
use futures::{SinkExt, StreamExt};

use crate::{
    state::{ClientSockets, ClientToSessions, ModifySession, SendToClient, Sessions},
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
            get_info::GetInfoResponse,
            get_pending_requests::GetPendingRequestsResponse,
        },
        common::{AckMessage, SessionStatus},
    },
};

pub async fn on_new_client_connection(
    ConnectInfo(ip): ConnectInfo<SocketAddr>,
    State(sessions): State<Sessions>,
    State(client_sockets): State<ClientSockets>,
    State(client_to_sessions): State<ClientToSessions>,
    ws: WebSocketUpgrade,
) -> Response {
    println!("New client connection from {}", ip);
    ws.on_upgrade(move |socket| {
        client_handler(socket, sessions, client_sockets, client_to_sessions)
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
    let client_id = loop {
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
        println!("msg {:?}", msg);
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
        println!("app_msg {:?}", app_msg);

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
                    .await;
                break connect_request.client_id;
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
                    let user_disconnected_event =
                        ServerToApp::UserDisconnectedEvent(UserDisconnectedEvent {});
                    let user_sessions = client_to_sessions.get_sessions(client_id.clone());
                    for session_id in user_sessions {
                        let mut session = match sessions.get_mut(&session_id) {
                            Some(session) => session,
                            None => continue,
                        };
                        session
                            .send_to_app(user_disconnected_event.clone())
                            .await
                            .unwrap_or_default();
                        session.update_status(SessionStatus::UserDisconnected);
                    }
                    return;
                }
            },
            None => {
                let user_disconnected_event =
                    ServerToApp::UserDisconnectedEvent(UserDisconnectedEvent {});
                let user_sessions = client_to_sessions.get_sessions(client_id.clone());
                for session_id in user_sessions {
                    let mut session = match sessions.get_mut(&session_id) {
                        Some(session) => session,
                        None => continue,
                    };
                    session
                        .send_to_app(user_disconnected_event.clone())
                        .await
                        .unwrap_or_default();
                    session.update_status(SessionStatus::UserDisconnected);
                }
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
                    let mut session = match sessions.get_mut(&session_id) {
                        Some(session) => session,
                        None => continue,
                    };
                    session
                        .send_to_app(user_disconnected_event.clone())
                        .await
                        .unwrap_or_default();
                    session.update_status(SessionStatus::UserDisconnected);
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
            ClientToServer::ConnectRequest(connect_request) => {
                println!("Connect request received {} ", connect_request.session_id);
                let mut session = sessions.get_mut(&connect_request.session_id).unwrap();
                // Insert user socket
                session.update_status(SessionStatus::ClientConnected);
                session.client_state.device = connect_request.device.clone();
                session.client_state.connected_public_keys = connect_request.public_keys.clone();
                session.client_state.client_id = Some(connect_request.client_id.clone());

                let client_reponse = ServerToClient::ConnectResponse(ConnectResponse {
                    response_id: connect_request.response_id,
                });
                client_sockets
                    .send_to_client(client_id.clone(), client_reponse)
                    .await
                    .unwrap();
                let app_event = ServerToApp::UserConnectedEvent(UserConnectedEvent {
                    public_keys: connect_request.public_keys,
                });
                session.send_to_app(app_event).await.unwrap();
                // Insert new session id into client_to_sessions
                client_to_sessions.add_session(
                    connect_request.client_id.clone(),
                    connect_request.session_id.clone(),
                );
            }
            ClientToServer::NewPayloadEventReply(new_payload_event_reply) => {
                let mut session = sessions
                    .get_mut(&new_payload_event_reply.session_id)
                    .unwrap();
                let _pending_request = session
                    .pending_requests
                    .remove(&new_payload_event_reply.request_id)
                    .unwrap();
                // Send to app
                let app_msg = ServerToApp::ResponsePayload(ResponsePayload {
                    response_id: new_payload_event_reply.request_id.clone(),
                    content: new_payload_event_reply.content.clone(),
                });
                session.send_to_app(app_msg).await.unwrap();

                let client_msg = ServerToClient::AckMessage(AckMessage {
                    response_id: new_payload_event_reply.response_id,
                });
                client_sockets
                    .send_to_client(client_id.clone(), client_msg)
                    .await
                    .unwrap();
            }
            ClientToServer::GetInfoRequest(get_info_request) => {
                let session = sessions.get_mut(&get_info_request.session_id).unwrap();
                let response = ServerToClient::GetInfoResponse(GetInfoResponse {
                    response_id: get_info_request.response_id,
                    network: session.network.clone(),
                    version: session.version.clone(),
                    app_metadata: session.app_state.metadata.clone(),
                });
                client_sockets
                    .send_to_client(client_id.clone(), response)
                    .await
                    .unwrap();
            }
            ClientToServer::GetPendingRequestsRequest(get_pending_requests_request) => {
                let session = sessions
                    .get_mut(&get_pending_requests_request.session_id)
                    .unwrap();
                let pending_requests = session
                    .pending_requests
                    .clone()
                    .iter()
                    .map(|v| v.clone())
                    .collect::<Vec<String>>();
                let response =
                    ServerToClient::GetPendingRequestsResponse(GetPendingRequestsResponse {
                        requests: pending_requests,
                        response_id: get_pending_requests_request.response_id,
                    });
                client_sockets
                    .send_to_client(client_id.clone(), response)
                    .await
                    .unwrap();
            }
            _ => {
                continue;
            }
        }
    }
}
