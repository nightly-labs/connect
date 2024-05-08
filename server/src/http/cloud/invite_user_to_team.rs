use super::utils::{custom_validate_team_id, validate_request};
use crate::{
    env::is_env_production,
    mailer::{
        mail_requests::{SendEmailRequest, TeamInviteNotification},
        mailer::Mailer,
    },
    middlewares::auth_middleware::UserId,
    statics::USERS_AMOUNT_LIMIT_PER_TEAM,
    structs::cloud::api_cloud_errors::CloudApiErrors,
};
use axum::{extract::State, http::StatusCode, Extension, Json};
use database::db::Db;
use garde::Validate;
use log::error;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, sync::Arc};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS, Validate)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpInviteUserToTeamRequest {
    #[garde(custom(custom_validate_team_id))]
    pub team_id: String,
    #[garde(email)]
    pub user_email: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct HttpInviteUserToTeamResponse {}

pub async fn invite_user_to_team(
    State(db): State<Arc<Db>>,
    State(mailer): State<Arc<Mailer>>,
    Extension(user_id): Extension<UserId>,
    Json(request): Json<HttpInviteUserToTeamRequest>,
) -> Result<Json<HttpInviteUserToTeamResponse>, (StatusCode, String)> {
    // Validate request
    validate_request(&request, &())?;

    // Get team data and perform checks
    match db.get_team_by_team_id(None, &request.team_id).await {
        Ok(Some(team)) => {
            // Check if user is a admin of this team
            if team.team_admin_id != user_id {
                return Err((
                    StatusCode::BAD_REQUEST,
                    CloudApiErrors::InsufficientPermissions.to_string(),
                ));
            }

            // Check team type
            if team.personal {
                return Err((
                    StatusCode::BAD_REQUEST,
                    CloudApiErrors::ActionForbiddenForPersonalTeam.to_string(),
                ));
            }

            // Check if team has at least one registered app
            match db.get_registered_apps_by_team_id(&request.team_id).await {
                Ok(apps) => {
                    if apps.is_empty() {
                        return Err((
                            StatusCode::BAD_REQUEST,
                            CloudApiErrors::TeamHasNoRegisteredApps.to_string(),
                        ));
                    }
                }
                Err(err) => {
                    error!("Failed to get registered apps by team id: {:?}", err);
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        CloudApiErrors::DatabaseError.to_string(),
                    ));
                }
            }

            // Get user data and perform checks
            let user = match db.get_user_by_email(&request.user_email).await {
                Ok(Some(user)) => user,
                Ok(None) => {
                    return Err((
                        StatusCode::BAD_REQUEST,
                        CloudApiErrors::UserDoesNotExist.to_string(),
                    ));
                }
                Err(err) => {
                    error!("Failed to get user by email: {:?}", err);
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        CloudApiErrors::DatabaseError.to_string(),
                    ));
                }
            };

            // Check if user already belongs to the team
            match db
                .get_teams_and_apps_membership_by_user_id(&user.user_id)
                .await
            {
                Ok(teams) => {
                    // This won't check if user has permissions to all apps in the team
                    if teams.iter().any(|(team_id, _)| team_id == &request.team_id) {
                        return Err((
                            StatusCode::BAD_REQUEST,
                            CloudApiErrors::UserAlreadyBelongsToTheTeam.to_string(),
                        ));
                    }
                }
                Err(err) => {
                    error!(
                        "Failed to get teams and apps membership by user id: {:?}",
                        err
                    );
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        CloudApiErrors::DatabaseError.to_string(),
                    ));
                }
            }

            // Check if limit of users in the team has been reached
            match db.get_privileges_by_team_id(&request.team_id).await {
                Ok(privileges) => {
                    let users = privileges
                        .iter()
                        .map(|privilege| privilege.user_id.clone())
                        .collect::<HashSet<String>>();

                    if users.len() >= USERS_AMOUNT_LIMIT_PER_TEAM {
                        return Err((
                            StatusCode::BAD_REQUEST,
                            CloudApiErrors::MaximumUsersPerTeamReached.to_string(),
                        ));
                    }

                    // Check the amount of invites to the team, team can only have one invite per user at a time
                    // Limit amount of possible invites to the team to the USERS_AMOUNT_LIMIT_PER_TEAM - amount of users in the team
                    // Get active invites
                    match db.get_invites_by_team_id(&request.team_id, true).await {
                        Ok(invites) => {
                            let invites = invites
                                .iter()
                                .map(|invite| invite.user_email.clone())
                                .collect::<HashSet<String>>();

                            // Additional check if user has already been invited to the team
                            if invites.contains(&request.user_email) {
                                return Err((
                                    StatusCode::BAD_REQUEST,
                                    CloudApiErrors::UserAlreadyInvitedToTheTeam.to_string(),
                                ));
                            }

                            // Check if invites limit has been reached
                            if invites.len() >= USERS_AMOUNT_LIMIT_PER_TEAM - users.len() {
                                return Err((
                                    StatusCode::BAD_REQUEST,
                                    CloudApiErrors::MaximumInvitesPerTeamReached.to_string(),
                                ));
                            }
                        }
                        Err(err) => {
                            error!("Failed to get invites by team id: {:?}", err);
                            return Err((
                                StatusCode::INTERNAL_SERVER_ERROR,
                                CloudApiErrors::DatabaseError.to_string(),
                            ));
                        }
                    }
                }
                Err(err) => {
                    error!("Failed to get privileges by team id: {:?}", err);
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        CloudApiErrors::DatabaseError.to_string(),
                    ));
                }
            }

            // Add invite to the team
            if let Err(err) = db
                .create_new_team_invite(&request.team_id, &request.user_email)
                .await
            {
                error!("Failed to invite user to the team: {:?}", err);
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    CloudApiErrors::DatabaseError.to_string(),
                ));
            }

            // Send email notification
            if is_env_production() {
                let request = SendEmailRequest::TeamInvite(TeamInviteNotification {
                    email: request.user_email.clone(),
                    team_name: team.team_name.clone(),
                    inviter_email: user.email.clone(),
                });

                // It doesn't matter if this fails
                if let Some(err) = mailer.handle_email_request(&request).error_message {
                    error!("Failed to send email: {:?}, request: {:?}", err, request);
                }
            }

            // Return response
            Ok(Json(HttpInviteUserToTeamResponse {}))
        }
        Ok(None) => {
            return Err((
                StatusCode::BAD_REQUEST,
                CloudApiErrors::TeamDoesNotExist.to_string(),
            ));
        }
        Err(err) => {
            error!("Failed to get team: {:?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::DatabaseError.to_string(),
            ));
        }
    }
}

#[cfg(feature = "cloud_integration_tests")]
#[cfg(test)]
mod tests {
    use crate::{
        env::JWT_SECRET,
        http::cloud::{
            invite_user_to_team::{HttpInviteUserToTeamRequest, HttpInviteUserToTeamResponse},
            register_new_app::HttpRegisterNewAppRequest,
        },
        statics::USERS_AMOUNT_LIMIT_PER_TEAM,
        structs::cloud::{
            api_cloud_errors::CloudApiErrors, cloud_http_endpoints::HttpCloudEndpoint,
        },
        test_utils::test_utils::{
            add_test_app, add_test_team, convert_response, create_test_app, generate_valid_name,
            invite_user_to_test_team, register_and_login_random_user,
        },
    };
    use axum::{
        body::Body,
        extract::ConnectInfo,
        http::{Method, Request},
    };
    use std::net::SocketAddr;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_invite_user_to_team() {
        let test_app = create_test_app(false).await;

        let (auth_token, _email, _password) = register_and_login_random_user(&test_app).await;

        // Register new team
        let team_name = generate_valid_name();
        let team_id = add_test_team(&team_name, &auth_token, &test_app, false)
            .await
            .unwrap();

        // Register app under the team
        let app_name = generate_valid_name();
        let request = HttpRegisterNewAppRequest {
            team_id: team_id.clone(),
            app_name: app_name.clone(),
        };

        // unwrap err as it should have failed
        let _ = add_test_app(&request, &auth_token, &test_app)
            .await
            .unwrap();

        // Register new user
        let (_test_user_auth_token, test_user_email, _test_user_password) =
            register_and_login_random_user(&test_app).await;

        // Invite user to the team
        let request = HttpInviteUserToTeamRequest {
            team_id: team_id.clone(),
            user_email: test_user_email.clone(),
        };

        let ip: ConnectInfo<SocketAddr> = ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 8080)));
        let json = serde_json::to_string(&request).unwrap();
        let auth = auth_token.encode(JWT_SECRET()).unwrap();

        let req = Request::builder()
            .method(Method::POST)
            .header("content-type", "application/json")
            .header("authorization", format!("Bearer {auth}"))
            .uri(format!(
                "/cloud/private{}",
                HttpCloudEndpoint::InviteUserToTeam.to_string()
            ))
            .extension(ip.clone())
            .body(Body::from(json))
            .unwrap();

        // Send request
        let response = test_app.clone().oneshot(req).await.unwrap();
        // Validate response
        convert_response::<HttpInviteUserToTeamResponse>(response)
            .await
            .unwrap();

        // Try to invite user to the team again, should fail as user is already invited to the team
        let request = HttpInviteUserToTeamRequest {
            team_id: team_id.clone(),
            user_email: test_user_email.clone(),
        };

        let json = serde_json::to_string(&request).unwrap();
        let auth = auth_token.encode(JWT_SECRET()).unwrap();

        let req = Request::builder()
            .method(Method::POST)
            .header("content-type", "application/json")
            .header("authorization", format!("Bearer {auth}"))
            .uri(format!(
                "/cloud/private{}",
                HttpCloudEndpoint::InviteUserToTeam.to_string()
            ))
            .extension(ip)
            .body(Body::from(json))
            .unwrap();

        // Send request
        let response = test_app.clone().oneshot(req).await.unwrap();
        // Validate response
        let err = convert_response::<HttpInviteUserToTeamResponse>(response)
            .await
            .unwrap_err();

        assert_eq!(
            err.to_string(),
            CloudApiErrors::UserAlreadyInvitedToTheTeam.to_string()
        );
    }

    #[tokio::test]
    async fn test_invite_user_to_team_team_not_found() {
        let test_app = create_test_app(false).await;

        let (auth_token, _email, _password) = register_and_login_random_user(&test_app).await;

        // Team does not exist
        let resp = invite_user_to_test_team(
            &i64::MAX.to_string(),
            &"test_user_email@gmail.com".to_string(),
            &auth_token,
            &test_app,
        )
        .await
        .unwrap_err();

        assert_eq!(
            resp.to_string(),
            CloudApiErrors::TeamDoesNotExist.to_string()
        );
    }

    #[tokio::test]
    async fn test_add_user_to_team_no_registered_apps() {
        let test_app = create_test_app(false).await;

        let (auth_token, _email, _password) = register_and_login_random_user(&test_app).await;

        // Register new team
        let team_name = generate_valid_name();
        let team_id = add_test_team(&team_name, &auth_token, &test_app, false)
            .await
            .unwrap();

        // Team does not have any apps
        let resp = invite_user_to_test_team(
            &team_id.to_string(),
            &"test_user_email@gmail.com".to_string(),
            &auth_token,
            &test_app,
        )
        .await
        .unwrap_err();

        assert_eq!(
            resp.to_string(),
            CloudApiErrors::TeamHasNoRegisteredApps.to_string()
        );
    }

    #[tokio::test]
    async fn test_invite_user_to_team_user_limit_reached() {
        let test_app = create_test_app(false).await;

        let (auth_token, _email, _password) = register_and_login_random_user(&test_app).await;

        // Register new team
        let team_name = generate_valid_name();
        let team_id = add_test_team(&team_name, &auth_token, &test_app, false)
            .await
            .unwrap();

        // Register app under the team
        let app_name = generate_valid_name();
        let request = HttpRegisterNewAppRequest {
            team_id: team_id.clone(),
            app_name: app_name.clone(),
        };

        let _ = add_test_app(&request, &auth_token, &test_app)
            .await
            .unwrap();

        // Add [USERS_AMOUNT_LIMIT_PER_TEAM] users to the team
        for _ in 1..USERS_AMOUNT_LIMIT_PER_TEAM {
            let (_, test_user_email, _) = register_and_login_random_user(&test_app).await;

            // Add user to the team
            invite_user_to_test_team(
                &team_id.to_string(),
                &test_user_email.to_string(),
                &auth_token,
                &test_app,
            )
            .await
            .unwrap();
        }

        // Try to add another user to the team, should fail as user limit has been reached
        let (_, test_user_email, _) = register_and_login_random_user(&test_app).await;

        // Add user to the team
        let resp = invite_user_to_test_team(
            &team_id.to_string(),
            &test_user_email.to_string(),
            &auth_token,
            &test_app,
        )
        .await
        .unwrap_err();

        assert_eq!(
            resp.to_string(),
            CloudApiErrors::MaximumInvitesPerTeamReached.to_string()
        );
    }

    #[tokio::test]
    async fn test_add_user_to_team_user_does_not_exist() {
        let test_app = create_test_app(false).await;

        let (auth_token, _email, _password) = register_and_login_random_user(&test_app).await;

        // Register new team
        let team_name = generate_valid_name();
        let team_id = add_test_team(&team_name, &auth_token, &test_app, false)
            .await
            .unwrap();

        // Register app under the team
        let app_name = generate_valid_name();
        let request = HttpRegisterNewAppRequest {
            team_id: team_id.clone(),
            app_name: app_name.clone(),
        };

        let _ = add_test_app(&request, &auth_token, &test_app)
            .await
            .unwrap();

        // Try to add non-existing user to the team, should fail as user limit has been reached

        // Add user to the team
        let resp = invite_user_to_test_team(
            &team_id.to_string(),
            &"non-existing-user@gmail.com".to_string(),
            &auth_token,
            &test_app,
        )
        .await
        .unwrap_err();

        assert_eq!(
            resp.to_string(),
            CloudApiErrors::UserDoesNotExist.to_string()
        );
    }
}
