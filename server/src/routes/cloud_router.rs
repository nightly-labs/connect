use crate::{
    http::cloud::{
        accept_team_invite::accept_team_invite,
        cancel_team_user_invite::cancel_team_user_invite,
        cancel_user_team_invite::cancel_user_team_invite,
        events::events::events,
        get_events::get_events,
        get_team_user_invites::get_team_user_invites,
        get_user_joined_teams::get_user_joined_teams,
        get_user_team_invites::get_user_team_invites,
        invite_user_to_team::invite_user_to_team,
        login::{login_with_google::login_with_google, login_with_password::login_with_password},
        register_new_app::register_new_app,
        register_new_team::register_new_team,
        register_with_password::register_with_password,
        remove_user_from_team::remove_user_from_team,
    },
    middlewares::auth_middleware::access_auth_middleware,
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
            &HttpCloudEndpoint::LoginWithGoogle.to_string(),
            post(login_with_google),
        )
        .route(
            &HttpCloudEndpoint::RegisterWithPassword.to_string(),
            post(register_with_password),
        )
        .route(&HttpCloudEndpoint::Events.to_string(), post(events))
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
            &HttpCloudEndpoint::InviteUserToTeam.to_string(),
            post(invite_user_to_team),
        )
        .route(
            &HttpCloudEndpoint::AcceptTeamInvite.to_string(),
            post(accept_team_invite),
        )
        .route(
            &HttpCloudEndpoint::RemoveUserFromTeam.to_string(),
            post(remove_user_from_team),
        )
        .route(
            &HttpCloudEndpoint::GetUserJoinedTeams.to_string(),
            get(get_user_joined_teams),
        )
        .route(
            &HttpCloudEndpoint::GetTeamUserInvites.to_string(),
            get(get_team_user_invites),
        )
        .route(
            &HttpCloudEndpoint::GetUserTeamInvites.to_string(),
            get(get_user_team_invites),
        )
        .route(
            &HttpCloudEndpoint::CancelTeamUserInvite.to_string(),
            post(cancel_team_user_invite),
        )
        .route(
            &HttpCloudEndpoint::CancelUserTeamInvite.to_string(),
            post(cancel_user_team_invite),
        )
        .route(&HttpCloudEndpoint::GetEvents.to_string(), get(get_events))
        .with_state(state)
}
