use crate::{
    http::cloud::{
        add_user_to_team::add_user_to_team, events::events,
        get_user_joined_teams::get_user_joined_teams, login_with_password::login_with_password,
        register_new_app::register_new_app, register_new_team::register_new_team,
        register_with_password::register_with_password,
        remove_user_from_team::remove_user_from_team,
    },
    middlewares::auth_middleware::access_auth_middleware,
    routes::cloud_router::events::events,
    state::ServerState,
    structs::cloud::cloud_http_endpoints::HttpCloudEndpoint,
};
use axum::{
    middleware,
    routing::{get, post},
    Router,
};

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
        .route(&HttpCloudEndpoint::EventsTest.to_string(), post(events))
        .with_state(state)
}

pub fn private_router(state: ServerState) -> Router<ServerState> {
    Router::new()
        .route(
            &HttpCloudEndpoint::RegisterNewApp.to_string(),
            post(register_new_app),
        )
        .route(
            &HttpCloudEndpoint::RegisterNewTeam.to_string(),
            post(register_new_team),
        )
        .route(
            &HttpCloudEndpoint::AddUserToTeam.to_string(),
            post(add_user_to_team),
        )
        .route(
            &HttpCloudEndpoint::RemoveUserFromTeam.to_string(),
            post(remove_user_from_team),
        )
        .route(
            &HttpCloudEndpoint::GetUserJoinedTeams.to_string(),
            get(get_user_joined_teams),
        )
        .with_state(state)
}
