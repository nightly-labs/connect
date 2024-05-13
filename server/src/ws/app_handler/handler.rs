use super::methods::{
    disconnect_session::disconnect_session, initialize_session::initialize_session_connection,
};
use crate::{
    cloud_state::CloudState,
    env::ONLY_RELAY_SERVICE,
    middlewares::origin_middleware::Origin,
    state::{
        ClientSockets, ClientToSessions, SendToClient, SessionToApp, SessionToAppMap, Sessions,
    },
    structs::{
        app_messages::{app_messages::AppToServer, initialize::InitializeRequest},
        client_messages::{client_messages::ServerToClient, new_payload_event::NewPayloadEvent},
        common::PendingRequest,
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
use database::structs::device_metadata::Device;
use futures::StreamExt;
use log::{debug, error, info, warn};
use std::{net::SocketAddr, sync::Arc};

pub async fn on_new_app_connection(
    ConnectInfo(ip): ConnectInfo<SocketAddr>,
    State(cloud): State<Option<Arc<CloudState>>>,
    Origin(origin): Origin,
    State(sessions): State<Sessions>,
    State(client_sockets): State<ClientSockets>,
    State(client_to_sessions): State<ClientToSessions>,
    State(session_to_app_map): State<SessionToAppMap>,
    ws: WebSocketUpgrade,
) -> Response {
    let ip = ip.to_string();
    ws.on_upgrade(move |socket| async move {
        debug!("OPEN app connection  from {}", ip);
        app_handler(
            socket,
            sessions,
            client_sockets,
            client_to_sessions,
            session_to_app_map,
            cloud,
            origin,
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
    cloud: Option<Arc<CloudState>>,
    origin: Option<String>,
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
                // If cloud is enabled, we will try to get app_id from the verified origin
                let app_id = match get_app_id(
                    ONLY_RELAY_SERVICE(),
                    &cloud,
                    &origin,
                    &init_data,
                    &session_to_app_map,
                )
                .await
                {
                    Ok(app_id) => app_id,
                    Err(_err) => {
                        // TODO explicit reject of the connection
                        return;
                    }
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
            AppToServer::InitializeRequest(_) => {
                // App should not send initialize message after the first one
                continue;
            }
        }
    }
}

async fn get_app_id(
    cloud_disabled: bool,
    cloud_state: &Option<Arc<CloudState>>,
    origin: &Option<String>,
    init_data: &InitializeRequest,
    session_to_app_map: &SessionToAppMap,
) -> Result<String, String> {
    // By default cloud is disabled, normal relay flow
    if cloud_disabled {
        return Ok(
            fetch_app_id_or_generate(&init_data.persistent_session_id, &session_to_app_map).await,
        );
    }

    // If cloud is enabled, we will try to get app_id from the verified origin if it was provided
    if let Some(cloud) = cloud_state {
        match origin {
            Some(origin) => {
                // Origin was provided, check if it is verified
                match cloud
                    .db
                    .get_domain_verification_by_domain_name(origin)
                    .await
                {
                    // Domain verification has been found
                    Ok(Some(domain_verification)) => {
                        // Check if domain is verified
                        if domain_verification.finished_at.is_some() {
                            // Check if app_id has been provided in initial data
                            match &init_data.app_id {
                                Some(app_id) => {
                                    // App id has been provided, check if it matches the verified app_id
                                    if app_id == &domain_verification.app_id {
                                        return Ok(app_id.clone());
                                    } else {
                                        info!(
                                            "App id mismatch: {} != {}",
                                            app_id, domain_verification.app_id
                                        );
                                        return Err("App id mismatch".to_string());
                                    }
                                }
                                // App id has not been provided, return the verified app_id
                                None => {
                                    return Ok(domain_verification.app_id);
                                }
                            }
                        } else {
                            info!("Domain verification has not been finished: {}", origin);
                            return Err("Domain verification has not been finished".to_string());
                        }
                    }
                    // Unverified domain, normal relay flow
                    Ok(None) | Err(_) => {
                        info!(
                            "Origin verification failed or error encountered: {}",
                            origin
                        );
                        return Ok(fetch_app_id_or_generate(
                            &init_data.persistent_session_id,
                            &session_to_app_map,
                        )
                        .await);
                    }
                }
            }
            None => {
                // If origin is missing, check if app id has been provided
                match &init_data.app_id {
                    Some(app_id) => {
                        // Check if provided app_id has already been registered
                        match cloud.db.get_registered_app_by_app_id(app_id).await {
                            Ok(Some(_)) => {
                                // app id is already used by a registered app, reject the connection by the origin
                                info!("App id already registered, origin not provided: {}", app_id);
                                return Err(
                                    "App id already registered, origin not provided".to_string()
                                );
                            }
                            // App id is not registered, or something has happened with the db, normal relay flow
                            Ok(None) | Err(_) => {
                                return Ok(fetch_app_id_or_generate(
                                    &init_data.persistent_session_id,
                                    &session_to_app_map,
                                )
                                .await);
                            }
                        }
                    }
                    None =>
                    // If app_id is not provided, normal relay flow
                    {
                        return Ok(fetch_app_id_or_generate(
                            &init_data.persistent_session_id,
                            &session_to_app_map,
                        )
                        .await);
                    }
                }
            }
        }
    } else {
        error!("Cloud feature is enabled but cloud state is not initialized");
        return Ok(
            fetch_app_id_or_generate(&init_data.persistent_session_id, &session_to_app_map).await,
        );
    }
}

async fn fetch_app_id_or_generate(
    session_id_option: &Option<String>,
    session_to_app_map: &SessionToAppMap,
) -> String {
    match session_id_option {
        Some(session_id) => session_to_app_map
            .get_app_id(session_id)
            .await
            .unwrap_or_else(|| {
                warn!("No app_id found for session: {}", session_id);
                uuid7::uuid7().to_string()
            }),
        None => uuid7::uuid7().to_string(),
    }
}
