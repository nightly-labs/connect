use super::{
    grafana_utils::remove_user_from_the_team::handle_grafana_remove_user_from_team,
    utils::{custom_validate_team_id, validate_request},
};
use crate::{
    env::is_env_production,
    mailer::{
        mail_requests::{SendEmailRequest, TeamRemovalNotification},
        mailer::Mailer,
    },
    middlewares::auth_middleware::UserId,
    structs::cloud::api_cloud_errors::CloudApiErrors,
};
use axum::{extract::State, http::StatusCode, Extension, Json};
use database::db::Db;
use garde::Validate;
use log::error;
use openapi::apis::configuration::Configuration;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS, Validate)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpRemoveUserFromTeamRequest {
    #[garde(custom(custom_validate_team_id))]
    pub team_id: String,
    #[garde(email)]
    pub user_email: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct HttpRemoveUserFromTeamResponse {}

pub async fn remove_user_from_team(
    State(db): State<Arc<Db>>,
    State(grafana_conf): State<Arc<Configuration>>,
    State(mailer): State<Arc<Mailer>>,
    Extension(user_id): Extension<UserId>,
    Json(request): Json<HttpRemoveUserFromTeamRequest>,
) -> Result<Json<HttpRemoveUserFromTeamResponse>, (StatusCode, String)> {
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
                    if !teams.iter().any(|(team_id, _)| team_id == &request.team_id) {
                        return Err((
                            StatusCode::BAD_REQUEST,
                            CloudApiErrors::UserDoesNotBelongsToTheTeam.to_string(),
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

            // Grafana, remove user from the team
            handle_grafana_remove_user_from_team(
                &grafana_conf,
                &request.team_id,
                &request.user_email,
            )
            .await?;

            // Remove user from the team
            if let Err(err) = db
                .remove_user_from_the_team(&user.user_id, &request.team_id)
                .await
            {
                error!("Failed to remove user from the team: {:?}", err);
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    CloudApiErrors::DatabaseError.to_string(),
                ));
            }

            // Send email notification
            if is_env_production() {
                let request = SendEmailRequest::TeamRemoval(TeamRemovalNotification {
                    email: user.email.clone(),
                    team_name: team.team_name.clone(),
                    remover_email: user_id,
                });

                // It doesn't matter if this fails
                if let Some(err) = mailer.handle_email_request(&request).error_message {
                    error!("Failed to send email: {:?}, request: {:?}", err, request);
                }
            }

            // Return response
            Ok(Json(HttpRemoveUserFromTeamResponse {}))
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

#[cfg(feature = "cloud_db_tests")]
#[cfg(test)]
mod tests {
    use crate::{
        env::JWT_SECRET,
        http::cloud::{
            register_new_app::HttpRegisterNewAppRequest,
            remove_user_from_team::{
                HttpRemoveUserFromTeamRequest, HttpRemoveUserFromTeamResponse,
            },
        },
        structs::cloud::{
            api_cloud_errors::CloudApiErrors, cloud_http_endpoints::HttpCloudEndpoint,
        },
        test_utils::test_utils::{
            add_test_app, add_test_team, add_user_to_test_team, convert_response, create_test_app,
            generate_valid_name, register_and_login_random_user, remove_user_from_test_team,
        },
    };
    use axum::{
        body::Body,
        extract::{ConnectInfo, Request},
        http::Method,
    };
    use std::net::SocketAddr;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_remove_user_from_team() {
        let test_app = create_test_app(false).await;

        let (auth_token, _email, _password) = register_and_login_random_user(&test_app).await;
        // Register new team
        let team_name = generate_valid_name();

        let team_id = add_test_team(&team_name, &auth_token, &test_app, false)
            .await
            .unwrap();

        println!("team_name: {:?}", team_name);
        println!("team_id: {:?}", team_id);

        // Register app under the team
        let app_name = generate_valid_name();
        let request = HttpRegisterNewAppRequest {
            team_id: team_id.to_string(),
            app_name: app_name.clone(),
        };

        // unwrap err as it should have failed
        let _ = add_test_app(&request, &auth_token, &test_app)
            .await
            .unwrap();

        // Register new user
        let (test_user_auth_token, test_user_email, _test_user_password) =
            register_and_login_random_user(&test_app).await;

        // Add user to the team
        add_user_to_test_team(
            &team_id.to_string(),
            &test_user_email,
            &auth_token,
            &test_user_auth_token,
            &test_app,
        )
        .await
        .unwrap();

        // Remove user from the team
        let request = HttpRemoveUserFromTeamRequest {
            team_id: team_id.to_string(),
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
                HttpCloudEndpoint::RemoveUserFromTeam.to_string()
            ))
            .extension(ip.clone())
            .body(Body::from(json))
            .unwrap();

        // Send request
        let response = test_app.clone().oneshot(req).await.unwrap();
        // Validate response
        convert_response::<HttpRemoveUserFromTeamResponse>(response)
            .await
            .unwrap();

        // Try to remove user from the team again, should fail as user is not in the team
        let json = serde_json::to_string(&request).unwrap();
        let auth = auth_token.encode(JWT_SECRET()).unwrap();

        let req = Request::builder()
            .method(Method::POST)
            .header("content-type", "application/json")
            .header("authorization", format!("Bearer {auth}"))
            .uri(format!(
                "/cloud/private{}",
                HttpCloudEndpoint::RemoveUserFromTeam.to_string()
            ))
            .extension(ip)
            .body(Body::from(json))
            .unwrap();

        // Send request
        let response = test_app.clone().oneshot(req).await.unwrap();
        // Validate response
        let err = convert_response::<HttpRemoveUserFromTeamResponse>(response)
            .await
            .unwrap_err();

        assert_eq!(
            err.to_string(),
            CloudApiErrors::UserDoesNotBelongsToTheTeam.to_string()
        );
    }

    #[tokio::test]
    async fn test_remove_user_from_team_team_not_found() {
        let test_app = create_test_app(false).await;

        let (auth_token, _email, _password) = register_and_login_random_user(&test_app).await;

        // Team does not exist
        let resp = remove_user_from_test_team(
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
    async fn test_remove_user_from_team_user_does_not_exist() {
        let test_app = create_test_app(false).await;

        let (auth_token, _email, _password) = register_and_login_random_user(&test_app).await;

        // Register new team
        let team_name = generate_valid_name();
        let team_id = add_test_team(&team_name, &auth_token, &test_app, false)
            .await
            .unwrap();

        // Team does not exist, use random uuid
        let resp = remove_user_from_test_team(
            &team_id.to_string(),
            &"test_user_email@gmail.com".to_string(),
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
