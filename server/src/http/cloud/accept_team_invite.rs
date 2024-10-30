use super::{
    grafana_utils::add_user_to_team::handle_grafana_add_user_to_team,
    utils::{custom_validate_team_id, validate_request},
};
use crate::{
    env::is_env_production, middlewares::auth_middleware::UserId,
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
pub struct HttpAcceptTeamInviteRequest {
    #[garde(custom(custom_validate_team_id))]
    pub team_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct HttpAcceptTeamInviteResponse {}

pub async fn accept_team_invite(
    State(db): State<Arc<Db>>,
    State(grafana_conf): State<Arc<Configuration>>,
    Extension(user_id): Extension<UserId>,
    Json(request): Json<HttpAcceptTeamInviteRequest>,
) -> Result<Json<HttpAcceptTeamInviteResponse>, (StatusCode, String)> {
    // Validate request
    validate_request(&request, &())?;

    // Get user data and perform checks
    let user = match db.get_user_by_user_id(&user_id).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Err((
                StatusCode::BAD_REQUEST,
                CloudApiErrors::UserDoesNotExist.to_string(),
            ));
        }
        Err(err) => {
            error!("Failed to get user by user_id: {:?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::DatabaseError.to_string(),
            ));
        }
    };

    // Just in case check if user already belongs to the team
    match db
        .get_teams_and_apps_membership_by_user_id(&user.user_id)
        .await
    {
        Ok(teams) => {
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

    // Check if user was invited to the team
    match db.get_invites_by_user_email(&user.email, true).await {
        Ok(invites) => {
            if !invites
                .iter()
                .any(|invite| invite.team_id == request.team_id)
            {
                return Err((
                    StatusCode::BAD_REQUEST,
                    CloudApiErrors::InviteNotFound.to_string(),
                ));
            }
        }
        Err(err) => {
            error!("Failed to get invites by user id: {:?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::DatabaseError.to_string(),
            ));
        }
    }

    let grafana_team_id = match db.get_team_by_team_id(None, &request.team_id).await {
        Ok(Some(team)) => team.grafana_id,
        Ok(None) => {
            return Err((
                StatusCode::BAD_REQUEST,
                CloudApiErrors::TeamDoesNotExist.to_string(),
            ));
        }
        Err(err) => {
            error!("Failed to get team by team_id: {:?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::DatabaseError.to_string(),
            ));
        }
    };
    // Grafana add user to the team
    if is_env_production() {
        if let Err(err) =
            handle_grafana_add_user_to_team(&grafana_conf, &grafana_team_id, &user.email).await
        {
            error!("Failed to add user to the team in grafana: {:?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::GrafanaError.to_string(),
            ));
        };
    }
    // Accept invite
    let mut tx = match db.connection_pool.begin().await {
        Ok(tx) => tx,
        Err(err) => {
            error!("Failed to start transaction: {:?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::DatabaseError.to_string(),
            ));
        }
    };

    // Accept invite
    if let Err(err) = db
        .accept_team_invite(&mut tx, &request.team_id, &user.email)
        .await
    {
        error!("Failed to accept team invite: {:?}", err);
        if let Err(err) = tx.rollback().await {
            error!("Failed to rollback transaction: {:?}", err);
        }

        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            CloudApiErrors::DatabaseError.to_string(),
        ));
    }

    // Add user to the team
    if let Err(err) = db
        .add_user_to_the_team(&mut tx, &user_id, &request.team_id)
        .await
    {
        error!("Failed to add user to the team: {:?}", err);
        if let Err(err) = tx.rollback().await {
            error!("Failed to rollback transaction: {:?}", err);
        }

        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            CloudApiErrors::DatabaseError.to_string(),
        ));
    }

    // Commit transaction
    if let Err(err) = tx.commit().await {
        error!("Failed to commit transaction: {:?}", err);
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            CloudApiErrors::DatabaseError.to_string(),
        ));
    }

    Ok(Json(HttpAcceptTeamInviteResponse {}))
}

#[cfg(feature = "cloud_integration_tests")]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::test_utils::accept_invite_to_test_team;
    use crate::{
        env::JWT_SECRET,
        http::cloud::register_new_app::HttpRegisterNewAppRequest,
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
        };

        // unwrap err as it should have failed
        let _ = add_test_app(&request, &auth_token, &test_app)
            .await
            .unwrap();

        // Register new user
        let (test_user_auth_token, test_user_email, _test_user_password) =
            register_and_login_random_user(&test_app).await;

        // Invite user to the team
        invite_user_to_test_team(&team_id, &test_user_email, &auth_token, &test_app)
            .await
            .unwrap();

        // Accept invite to the team
        let request = HttpAcceptTeamInviteRequest {
            team_id: team_id.clone(),
        };

        let ip: ConnectInfo<SocketAddr> = ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 8080)));
        let json = serde_json::to_string(&request).unwrap();
        let auth = test_user_auth_token.encode(JWT_SECRET()).unwrap();

        let req = Request::builder()
            .method(Method::POST)
            .header("content-type", "application/json")
            .header("authorization", format!("Bearer {auth}"))
            .uri(format!(
                "/cloud/private{}",
                HttpCloudEndpoint::AcceptTeamInvite.to_string()
            ))
            .extension(ip.clone())
            .body(Body::from(json))
            .unwrap();

        // Send request
        let response = test_app.clone().oneshot(req).await.unwrap();
        // Validate response
        convert_response::<HttpAcceptTeamInviteResponse>(response)
            .await
            .unwrap();

        // Try to add user to the team again, should fail as user is already in the team
        let request = HttpAcceptTeamInviteRequest {
            team_id: team_id.clone(),
        };

        let json = serde_json::to_string(&request).unwrap();
        let auth = test_user_auth_token.encode(JWT_SECRET()).unwrap();

        let req = Request::builder()
            .method(Method::POST)
            .header("content-type", "application/json")
            .header("authorization", format!("Bearer {auth}"))
            .uri(format!(
                "/cloud/private{}",
                HttpCloudEndpoint::AcceptTeamInvite.to_string()
            ))
            .extension(ip)
            .body(Body::from(json))
            .unwrap();

        // Send request
        let response = test_app.clone().oneshot(req).await.unwrap();
        // Validate response
        let err = convert_response::<HttpAcceptTeamInviteResponse>(response)
            .await
            .unwrap_err();

        assert_eq!(
            err.to_string(),
            CloudApiErrors::UserAlreadyBelongsToTheTeam.to_string()
        );
    }

    #[tokio::test]
    async fn test_accept_no_invite_found() {
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
        let (test_user_auth_token, _test_user_email, _test_user_password) =
            register_and_login_random_user(&test_app).await;

        // Accept invite to the team
        let err = accept_invite_to_test_team(&team_id, &test_user_auth_token, &test_app)
            .await
            .unwrap_err();

        assert_eq!(err.to_string(), CloudApiErrors::InviteNotFound.to_string());
    }
}
