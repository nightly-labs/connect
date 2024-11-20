use crate::{
    http::cloud::{
        accept_team_invite::accept_team_invite,
        add_passkey_finish::add_passkey_finish,
        add_passkey_start::add_passkey_start,
        cancel_team_user_invite::cancel_team_user_invite,
        cancel_user_team_invite::cancel_user_team_invite,
        change_user_privileges::change_user_privileges,
        delete_account_finish::delete_account_finish,
        delete_account_start::delete_account_start,
        delete_app::delete_app,
        delete_passkey::delete_passkey,
        delete_team::delete_team,
        domains::{
            cancel_pending_domain_request::cancel_pending_domain_request,
            remove_whitelisted_domain::remove_whitelisted_domain,
            verify_domain_finish::verify_domain_finish, verify_domain_start::verify_domain_start,
        },
        events::events::events,
        get_events::get_events,
        get_passkey_challenge::get_passkey_challenge,
        get_team_metadata::get_team_metadata,
        get_team_user_invites::get_team_user_invites,
        get_team_users_privileges::get_team_users_privileges,
        get_user_joined_teams::get_user_joined_teams,
        get_user_metadata::get_user_metadata,
        get_user_team_invites::get_user_team_invites,
        invite_user_to_team::invite_user_to_team,
        leave_team::leave_team,
        login::{
            login_with_google::login_with_google,
            login_with_passkey_finish::login_with_passkey_finish,
            login_with_passkey_start::login_with_passkey_start,
            login_with_password::login_with_password, refresh_token::refresh_token,
        },
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
            reset_passkey_finish::reset_passkey_finish, reset_passkey_start::reset_passkey_start,
            reset_password_finish::reset_password_finish,
            reset_password_start::reset_password_start,
        },
        verify_code::verify_code,
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
            &HttpCloudEndpoint::LoginWithPasskeyStart.to_string(),
            post(login_with_passkey_start),
        )
        .route(
            &HttpCloudEndpoint::LoginWithPasskeyFinish.to_string(),
            post(login_with_passkey_finish),
        )
        .route(
            &HttpCloudEndpoint::RefreshToken.to_string(),
            post(refresh_token),
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
            &HttpCloudEndpoint::RegisterWithPasskeyFinish.to_string(),
            post(register_with_passkey_finish),
        )
        .route(
            &HttpCloudEndpoint::ResetPasskeyStart.to_string(),
            post(reset_passkey_start),
        )
        .route(
            &HttpCloudEndpoint::ResetPasskeyFinish.to_string(),
            post(reset_passkey_finish),
        )
        .route(
            &HttpCloudEndpoint::VerifyCode.to_string(),
            post(verify_code),
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
        .route(
            &HttpCloudEndpoint::CancelPendingDomainVerification.to_string(),
            post(cancel_pending_domain_request),
        )
        .route(
            &HttpCloudEndpoint::GetPasskeyChallenge.to_string(),
            get(get_passkey_challenge),
        )
        .route(
            &HttpCloudEndpoint::DeletePasskey.to_string(),
            post(delete_passkey),
        )
        .route(
            &HttpCloudEndpoint::AddPasskeyStart.to_string(),
            post(add_passkey_start),
        )
        .route(
            &HttpCloudEndpoint::AddPasskeyFinish.to_string(),
            post(add_passkey_finish),
        )
        .route(
            &HttpCloudEndpoint::GetUserMetadata.to_string(),
            get(get_user_metadata),
        )
        .route(
            &HttpCloudEndpoint::GetTeamMetadata.to_string(),
            get(get_team_metadata),
        )
        .route(
            &HttpCloudEndpoint::GetTeamUserPrivileges.to_string(),
            get(get_team_users_privileges),
        )
        .route(
            &HttpCloudEndpoint::ChangeUserPrivileges.to_string(),
            post(change_user_privileges),
        )
        .route(&HttpCloudEndpoint::DeleteApp.to_string(), post(delete_app))
        .route(&HttpCloudEndpoint::LeaveTeam.to_string(), post(leave_team))
        .route(
            &HttpCloudEndpoint::DeleteTeam.to_string(),
            post(delete_team),
        )
        .route(
            &HttpCloudEndpoint::DeleteAccountStart.to_string(),
            post(delete_account_start),
        )
        .route(
            &HttpCloudEndpoint::DeleteAccountFinish.to_string(),
            post(delete_account_finish),
        )
        .with_state(state)
}
