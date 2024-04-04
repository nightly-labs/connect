use crate::{
    http::cloud::{
        accept_team_invite::accept_team_invite,
        cancel_team_user_invite::cancel_team_user_invite,
        cancel_user_team_invite::cancel_user_team_invite,
        domains::{
            remove_whitelisted_domain::remove_whitelisted_domain,
            verify_domain_finish::verify_domain_finish, verify_domain_start::verify_domain_start,
        },
        events::events::events,
        get_events::get_events,
        get_team_user_invites::get_team_user_invites,
        get_user_joined_teams::get_user_joined_teams,
        get_user_team_invites::get_user_team_invites,
        invite_user_to_team::invite_user_to_team,
        login::{login_with_google::login_with_google, login_with_password::login_with_password},
        register::{
            register_with_passkey_finish::register_with_passkey_finish,
            register_with_passkey_start::register_with_passkey_start,
            register_with_password_finish::register_with_password_finish,
            register_with_password_start::register_with_password_start,
        },
        register_new_app::register_new_app,
        register_new_team::register_new_team,
        remove_user_from_team::remove_user_from_team,
        reset_credentials::{
            reset_password_finish::reset_password_finish,
            reset_password_start::reset_password_start,
        },
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
            &HttpCloudEndpoint::RegisterWithPasswordStart.to_string(),
            post(register_with_password_start),
        )
        .route(
            &HttpCloudEndpoint::RegisterWithPasswordFinish.to_string(),
            post(register_with_password_finish),
        )
        .route(
            &HttpCloudEndpoint::ResetPasswordStart.to_string(),
            post(reset_password_start),
        )
        .route(
            &HttpCloudEndpoint::ResetPasswordFinish.to_string(),
            post(reset_password_finish),
        )
        .route(
            &HttpCloudEndpoint::RegisterWithPasskeyStart.to_string(),
            post(register_with_passkey_start),
        )
        .route(
            &HttpCloudEndpoint::ResetPasswordFinish.to_string(),
            post(register_with_passkey_finish),
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
        .route(
            &HttpCloudEndpoint::VerifyDomainStart.to_string(),
            post(verify_domain_start),
        )
        .route(
            &HttpCloudEndpoint::VerifyDomainFinish.to_string(),
            post(verify_domain_finish),
        )
        .route(
            &HttpCloudEndpoint::RemoveWhitelistedDomain.to_string(),
            post(remove_whitelisted_domain),
        )
        .with_state(state)
}
