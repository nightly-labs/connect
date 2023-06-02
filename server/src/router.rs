use std::time::Duration;

use axum::{
    error_handling::HandleErrorLayer,
    routing::{get, post},
    Router,
};
use tower::ServiceBuilder;

use crate::{
    app::app_handler::on_new_app_connection,
    client::{
        client_handler::on_new_client_connection, drop_sessions::drop_sessions,
        get_pending_requests::get_pending_requests, get_sessions::get_sessions,
        resolve_sign_messages_request::resolve_sign_messages_request,
        resolve_sign_transactions_request::resolve_sign_transactions_request,
    },
    handle_error::handle_error,
    state::ServerState,
};

pub async fn get_router() -> Router {
    let state = ServerState {
        sessions: Default::default(),
        client_to_sessions: Default::default(),
    };
    return Router::new()
        .route("/client", get(on_new_client_connection))
        .route("/app", get(on_new_app_connection))
        .route("/get_sessions", post(get_sessions))
        .route("/drop_sessions", post(drop_sessions))
        .route("/get_pending_requests", post(get_pending_requests))
        .route(
            "/resolve_sign_messages_request",
            post(resolve_sign_messages_request),
        )
        .route(
            "/resolve_sign_transactions_request",
            post(resolve_sign_transactions_request),
        )
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(handle_error))
                .load_shed()
                .concurrency_limit(1024)
                .timeout(Duration::from_secs(10))
                .into_inner(),
        )
        .with_state(state);
}
