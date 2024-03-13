use crate::{
    middlewares::auth_middleware::UserId,
    statics::USERS_AMOUNT_LIMIT_PER_TEAM,
    structs::api_cloud_errors::CloudApiErrors,
    utils::{custom_validate_uuid, validate_request},
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
pub struct HttpAddUserToTeamRequest {
    #[garde(custom(custom_validate_uuid))]
    pub team_id: String,
    #[garde(email)]
    pub user_email: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct HttpAddUserToTeamResponse {}

pub async fn add_user_to_team(
    State(db): State<Option<Arc<Db>>>,
    Extension(user_id): Extension<UserId>,
    Json(request): Json<HttpAddUserToTeamRequest>,
) -> Result<Json<HttpAddUserToTeamResponse>, (StatusCode, String)> {
    // Db connection has already been checked in the middleware
    let db = db.as_ref().ok_or((
        StatusCode::INTERNAL_SERVER_ERROR,
        CloudApiErrors::CloudFeatureDisabled.to_string(),
    ))?;

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
                }
                Err(err) => {
                    error!("Failed to get privileges by team id: {:?}", err);
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

            // Add user to the team
            match db
                .add_user_to_the_team(&user.user_id, &request.team_id)
                .await
            {
                Ok(_) => {
                    return Ok(Json(HttpAddUserToTeamResponse {}));
                }
                Err(err) => {
                    error!("Failed to add user to the team: {:?}", err);
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        CloudApiErrors::DatabaseError.to_string(),
                    ));
                }
            }
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

#[cfg(test)]
mod tests {
    use crate::{
        env::JWT_SECRET,
        http::cloud::{
            add_user_to_team::{HttpAddUserToTeamRequest, HttpAddUserToTeamResponse},
            register_new_app::HttpRegisterNewAppRequest,
        },
        statics::USERS_AMOUNT_LIMIT_PER_TEAM,
        structs::{api_cloud_errors::CloudApiErrors, cloud_http_endpoints::HttpCloudEndpoint},
        test_utils::test_utils::{
            add_test_app, add_test_team, add_user_to_test_team, convert_response, create_test_app,
            generate_valid_name, register_and_login_random_user,
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
    async fn test_add_user_to_team() {
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
            whitelisted_domains: vec![],
            ack_public_keys: vec![],
        };

        // unwrap err as it should have failed
        let _ = add_test_app(&request, &auth_token, &test_app)
            .await
            .unwrap();

        // Register new user
        let (_test_user_auth_token, test_user_email, _test_user_password) =
            register_and_login_random_user(&test_app).await;

        // Add user to the team
        let request = HttpAddUserToTeamRequest {
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
                HttpCloudEndpoint::AddUserToTeam.to_string()
            ))
            .extension(ip.clone())
            .body(Body::from(json))
            .unwrap();

        // Send request
        let response = test_app.clone().oneshot(req).await.unwrap();
        // Validate response
        convert_response::<HttpAddUserToTeamResponse>(response)
            .await
            .unwrap();

        // Try to add user to the team again, should fail as user is already in the team
        let request = HttpAddUserToTeamRequest {
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
                HttpCloudEndpoint::AddUserToTeam.to_string()
            ))
            .extension(ip)
            .body(Body::from(json))
            .unwrap();

        // Send request
        let response = test_app.clone().oneshot(req).await.unwrap();
        // Validate response
        let err = convert_response::<HttpAddUserToTeamResponse>(response)
            .await
            .unwrap_err();

        assert_eq!(
            err.to_string(),
            CloudApiErrors::UserAlreadyBelongsToTheTeam.to_string()
        );
    }

    #[tokio::test]
    async fn test_add_user_to_team_team_not_found() {
        let test_app = create_test_app(false).await;

        let (auth_token, _email, _password) = register_and_login_random_user(&test_app).await;

        // Team does not exist, use random uuid
        let resp = add_user_to_test_team(
            &uuid7::uuid7().to_string(),
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

        // Team does not exist, use random uuid
        let resp = add_user_to_test_team(
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
    async fn test_add_user_to_team_user_limit_reached() {
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
            whitelisted_domains: vec![],
            ack_public_keys: vec![],
        };

        let _ = add_test_app(&request, &auth_token, &test_app)
            .await
            .unwrap();

        // Add [USERS_AMOUNT_LIMIT_PER_TEAM] users to the team
        for _ in 1..USERS_AMOUNT_LIMIT_PER_TEAM {
            let (_, test_user_email, _) = register_and_login_random_user(&test_app).await;

            // Add user to the team
            add_user_to_test_team(
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
        let resp = add_user_to_test_team(
            &team_id.to_string(),
            &test_user_email.to_string(),
            &auth_token,
            &test_app,
        )
        .await
        .unwrap_err();

        assert_eq!(
            resp.to_string(),
            CloudApiErrors::MaximumUsersPerTeamReached.to_string()
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
            whitelisted_domains: vec![],
            ack_public_keys: vec![],
        };

        let _ = add_test_app(&request, &auth_token, &test_app)
            .await
            .unwrap();

        // Try to add non-existing user to the team, should fail as user limit has been reached

        // Add user to the team
        let resp = add_user_to_test_team(
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
