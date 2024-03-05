use crate::{
    http::statistics::{
        get_registered_apps::get_registered_apps, register_new_app::register_new_app,
        register_new_user::register_new_user,
    },
    state::ServerState,
    structs::stats_http_endpoints::HttpStatsEndpoint,
};
use axum::{
    routing::{get, post},
    Router,
};

pub fn stats_router(state: ServerState) -> Router<ServerState> {
    Router::new()
        .route(
            &HttpStatsEndpoint::RegisterNewApp.to_string(),
            post(register_new_app),
        )
        .route(
            &HttpStatsEndpoint::RegisterNewUser.to_string(),
            post(register_new_user),
        )
        .route(
            &HttpStatsEndpoint::GetRegisteredApps.to_string(),
            get(get_registered_apps),
        )
        .with_state(state)
}
