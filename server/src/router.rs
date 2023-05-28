use std::time::Duration;

use axum::{error_handling::HandleErrorLayer, routing::get, Router};
use tower::ServiceBuilder;

use crate::{
    app::app_handler::on_new_app_connection, client::client_handler::on_new_client_connection,
    handle_error::handle_error, state::ServerState,
};

pub async fn get_router() -> Router {
    let state = ServerState {
        sessions: Default::default(),
    };
    return Router::new()
        .route("/client", get(on_new_client_connection))
        .route("/app", get(on_new_app_connection))
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
