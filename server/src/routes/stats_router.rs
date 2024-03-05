use crate::{
    auth::auth_middleware::access_auth_middleware,
    http::statistics::{
        get_registered_apps::get_registered_apps, login_with_password::login_with_password,
        register_new_app::register_new_app, register_new_user::register_new_user,
    },
    state::ServerState,
    structs::stats_http_endpoints::HttpStatsEndpoint,
};
use axum::{
    middleware,
    routing::{get, post},
    Router,
};

pub fn stats_router(state: ServerState) -> Router<ServerState> {
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
            &HttpStatsEndpoint::LoginWithPassword.to_string(),
            post(login_with_password),
        )
        .route(
            &HttpStatsEndpoint::RegisterNewApp.to_string(),
            post(register_new_app),
        )
        .route(
            &HttpStatsEndpoint::RegisterNewUser.to_string(),
            post(register_new_user),
        )
        .with_state(state)
}

pub fn private_router(state: ServerState) -> Router<ServerState> {
    Router::new()
        .route(
            &HttpStatsEndpoint::GetRegisteredApps.to_string(),
            get(get_registered_apps),
        )
        .with_state(state)
}
