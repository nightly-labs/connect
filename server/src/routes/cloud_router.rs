use crate::{
    auth::auth_middleware::access_auth_middleware,
    http::cloud::{
        login_with_password::login_with_password, register_new_app::register_new_app,
        register_with_password::register_with_password,
    },
    state::ServerState,
    structs::cloud_http_endpoints::HttpCloudEndpoint,
};
use axum::{middleware, routing::post, Router};

pub fn cloud_router(state: ServerState) -> Router<ServerState> {
    Router::new()
        .nest("/public", public_router(state.clone()))
        .nest(
            "/private",
            private_router(state.clone()).route_layer(middleware::from_fn_with_state(
                state.clone(),
                access_auth_middleware,
            )),
        )
        .with_state(state)
}

pub fn public_router(state: ServerState) -> Router<ServerState> {
    Router::new()
        .route(
            &HttpCloudEndpoint::LoginWithPassword.to_string(),
            post(login_with_password),
        )
        .route(
            &HttpCloudEndpoint::RegisterWithPassword.to_string(),
            post(register_with_password),
        )
        .with_state(state)
}

pub fn private_router(state: ServerState) -> Router<ServerState> {
    Router::new()
        .route(
            &HttpCloudEndpoint::RegisterNewApp.to_string(),
            post(register_new_app),
        )
        .with_state(state)
}
