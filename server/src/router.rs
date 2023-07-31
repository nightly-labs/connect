use std::time::Duration;

use axum::{
    error_handling::HandleErrorLayer,
    routing::{get, post},
    Router,
};
use tower::ServiceBuilder;
use tracing_subscriber::EnvFilter;

use crate::{
    app::app_handler::on_new_app_connection,
    client::{
        client_handler::on_new_client_connection, connect_session::connect_session,
        drop_sessions::drop_sessions, get_pending_request::get_pending_request,
        get_pending_requests::get_pending_requests, get_session_info::get_session_info,
        get_sessions::get_sessions, get_wallets_metadata::get_wallets_metadata,
        resolve_request::resolve_request,
    },
    handle_error::handle_error,
    sesssion_cleaner::start_cleaning_sessions,
    state::ServerState,
    structs::http_endpoints::HttpEndpoint,
    utils::get_cors,
};
use tower_http::trace::TraceLayer;
pub async fn get_router() -> Router {
    let state = ServerState {
        sessions: Default::default(),
        client_to_sessions: Default::default(),
        client_to_sockets: Default::default(),
    };
    // Start cleaning outdated sessions
    start_cleaning_sessions(state.sessions.clone(), state.client_to_sessions.clone());
    let cors = get_cors();

    let filter: EnvFilter = "debug,tower_http=trace,hyper=warn"
        .parse()
        .expect("filter should parse");

    tracing_subscriber::fmt().with_env_filter(filter).init();

    return Router::new()
        .route("/client", get(on_new_client_connection))
        .route("/app", get(on_new_app_connection))
        .route(
            &HttpEndpoint::GetSessionInfo.to_string(),
            post(get_session_info),
        )
        .route(
            &HttpEndpoint::ConnectSession.to_string(),
            post(connect_session),
        )
        .route(&HttpEndpoint::GetSessions.to_string(), post(get_sessions))
        .route(&HttpEndpoint::DropSessions.to_string(), post(drop_sessions))
        .route(
            &HttpEndpoint::GetPendingRequests.to_string(),
            post(get_pending_requests),
        )
        .route(
            &HttpEndpoint::GetPendingRequest.to_string(),
            post(get_pending_request),
        )
        .route(
            &HttpEndpoint::ResolveRequest.to_string(),
            post(resolve_request),
        )
        .route(
            &HttpEndpoint::GetWalletsMetadata.to_string(),
            get(get_wallets_metadata),
        )
        .with_state(state)
        .layer(TraceLayer::new_for_http())
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(handle_error))
                .timeout(Duration::from_secs(10)),
        )
        .layer(cors);
}
