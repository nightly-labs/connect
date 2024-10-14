use super::{
    grafana_utils::create_new_team::handle_grafana_create_new_team,
    utils::{custom_validate_name, validate_request},
};
use crate::{
    env::is_env_production, middlewares::auth_middleware::UserId,
    statics::TEAMS_AMOUNT_LIMIT_PER_USER, structs::cloud::api_cloud_errors::CloudApiErrors,
};
use axum::{extract::State, http::StatusCode, Extension, Json};
use database::{
    db::Db,
    tables::{team::table_struct::Team, utils::get_current_datetime},
};
use garde::Validate;
use log::error;
use openapi::apis::configuration::Configuration;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS, Validate)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpRegisterNewTeamRequest {
    #[garde(custom(custom_validate_name))]
    pub team_name: String,
    #[garde(skip)]
    pub personal: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpRegisterNewTeamResponse {
    pub team_id: String,
}

pub async fn register_new_team(
    State(db): State<Arc<Db>>,
    State(grafana_conf): State<Arc<Configuration>>,
    Extension(user_id): Extension<UserId>,
    Json(request): Json<HttpRegisterNewTeamRequest>,
) -> Result<Json<HttpRegisterNewTeamResponse>, (StatusCode, String)> {
    // Validate request
    validate_request(&request, &())?;

    // First check if user is creating a new team
    // Get team data and perform checks
    match db
        .get_team_by_team_name_and_admin_id(&request.team_name, &user_id)
        .await
    {
        Ok(team) => {
            if let Some(team) = team {
                if team.deactivated_at == None {
                    return Err((
                        StatusCode::BAD_REQUEST,
                        CloudApiErrors::TeamAlreadyExists.to_string(),
                    ));
                }
            }

            // Check how many teams the user has
            match db.get_user_created_teams_without_personal(&user_id).await {
                Ok(teams) => {
                    if teams.len() >= TEAMS_AMOUNT_LIMIT_PER_USER {
                        return Err((
                            StatusCode::BAD_REQUEST,
                            "User has reached the maximum number of teams".to_string(),
                        ));
                    }
                }
                Err(err) => {
                    error!(
                        "Failed to get user created teams without personal: {:?}",
                        err
                    );
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        CloudApiErrors::DatabaseError.to_string(),
                    ));
                }
            }

            // Check if user already has a personal team
            if request.personal {
                match db.get_personal_team_by_admin_id(&user_id).await {
                    Ok(Some(_)) => {
                        return Err((
                            StatusCode::BAD_REQUEST,
                            CloudApiErrors::PersonalTeamAlreadyExists.to_string(),
                        ));
                    }
                    Ok(None) => {
                        // Continue
                    }
                    Err(err) => {
                        error!("Failed to get personal team by admin id: {:?}", err);
                        return Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            CloudApiErrors::DatabaseError.to_string(),
                        ));
                    }
                }
            }

            // Get team admin email
            let admin_email = match db.get_user_by_user_id(&user_id).await {
                Ok(Some(user)) => user.email,
                Ok(None) => {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        CloudApiErrors::DatabaseError.to_string(),
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
            let mut grafana_team_id: i64 = 0;
            // Grafana, add new team
            if is_env_production() {
                grafana_team_id = match handle_grafana_create_new_team(
                    &grafana_conf,
                    &admin_email,
                    &request.team_name,
                )
                .await
                {
                    Ok(id) => id,
                    Err(err) => {
                        error!("Failed to create team in grafana: {:?}", err);
                        return Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            CloudApiErrors::GrafanaError.to_string(),
                        ));
                    }
                }
            }

            // Create a new team
            let team = Team {
                team_id: grafana_team_id.to_string(),
                team_name: request.team_name.clone(),
                team_admin_id: user_id.clone(),
                subscription: None,
                personal: request.personal,
                registration_timestamp: get_current_datetime(),
                deactivated_at: None,
            };

            if let Err(err) = db.create_new_team(&team).await {
                error!("Failed to create team {:?}", err);
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    CloudApiErrors::DatabaseError.to_string(),
                ));
            }

            return Ok(Json(HttpRegisterNewTeamResponse {
                team_id: grafana_team_id.to_string(),
            }));
        }
        Err(err) => {
            error!("Failed to get team by team name and admin id: {:?}", err);
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
        http::cloud::register_new_team::{HttpRegisterNewTeamRequest, HttpRegisterNewTeamResponse},
        structs::cloud::{
            api_cloud_errors::CloudApiErrors, cloud_http_endpoints::HttpCloudEndpoint,
        },
        test_utils::test_utils::{
            convert_response, create_test_app, generate_valid_name, register_and_login_random_user,
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
    async fn test_register_new_normal_team() {
        let test_app = create_test_app(false).await;

        let (auth_token, _email, _password) = register_and_login_random_user(&test_app).await;

        // Register new team
        let first_team_name = generate_valid_name();
        let request = HttpRegisterNewTeamRequest {
            team_name: first_team_name.clone(),
            personal: false,
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
                HttpCloudEndpoint::RegisterNewTeam.to_string()
            ))
            .extension(ip)
            .body(Body::from(json))
            .unwrap();

        // Send request
        let response = test_app.clone().oneshot(req).await.unwrap();
        // Validate response
        convert_response::<HttpRegisterNewTeamResponse>(response)
            .await
            .unwrap();

        // Try to register the same team again, should fail
        let json = serde_json::to_string(&request).unwrap();
        let req = Request::builder()
            .method(Method::POST)
            .header("content-type", "application/json")
            .header("authorization", format!("Bearer {auth}"))
            .uri(format!(
                "/cloud/private{}",
                HttpCloudEndpoint::RegisterNewTeam.to_string()
            ))
            .extension(ip)
            .body(Body::from(json))
            .unwrap();

        // Send request
        let response = test_app.clone().oneshot(req).await.unwrap();
        // Validate response
        let err = convert_response::<HttpRegisterNewTeamResponse>(response)
            .await
            .unwrap_err();

        assert_eq!(
            err.to_string(),
            CloudApiErrors::TeamAlreadyExists.to_string()
        );
    }

    #[tokio::test]
    async fn test_register_new_personal_team() {
        let test_app = create_test_app(false).await;

        let (auth_token, _email, _password) = register_and_login_random_user(&test_app).await;

        // Register new team
        let first_team_name = generate_valid_name();
        let request = HttpRegisterNewTeamRequest {
            team_name: first_team_name.clone(),
            personal: true,
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
                HttpCloudEndpoint::RegisterNewTeam.to_string()
            ))
            .extension(ip)
            .body(Body::from(json))
            .unwrap();

        // Send request
        let response = test_app.clone().oneshot(req).await.unwrap();
        // Validate response
        convert_response::<HttpRegisterNewTeamResponse>(response)
            .await
            .unwrap();

        // Try to register the new personal team, should fail
        let second_team_name = "MySecondTeam".to_string();
        let request = HttpRegisterNewTeamRequest {
            team_name: second_team_name.clone(),
            personal: true,
        };

        let json = serde_json::to_string(&request).unwrap();
        let req = Request::builder()
            .method(Method::POST)
            .header("content-type", "application/json")
            .header("authorization", format!("Bearer {auth}"))
            .uri(format!(
                "/cloud/private{}",
                HttpCloudEndpoint::RegisterNewTeam.to_string()
            ))
            .extension(ip)
            .body(Body::from(json))
            .unwrap();

        // Send request
        let response = test_app.clone().oneshot(req).await.unwrap();
        // Validate response
        let err = convert_response::<HttpRegisterNewTeamResponse>(response)
            .await
            .unwrap_err();

        assert_eq!(
            err.to_string(),
            CloudApiErrors::PersonalTeamAlreadyExists.to_string()
        );
    }

    #[tokio::test]
    async fn test_invalid_team_name() {
        let test_app = create_test_app(false).await;

        let (auth_token, _email, _password) = register_and_login_random_user(&test_app).await;

        let request = HttpRegisterNewTeamRequest {
            team_name: generate_valid_name() + "1827389012hds012hd!!>>>>>>>>.",
            personal: true,
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
                HttpCloudEndpoint::RegisterNewTeam.to_string()
            ))
            .extension(ip)
            .body(Body::from(json))
            .unwrap();

        // Send request
        let response = test_app.clone().oneshot(req).await.unwrap();
        // Validate response
        let err = convert_response::<HttpRegisterNewTeamResponse>(response)
            .await
            .unwrap_err();

        assert_eq!(err.to_string(), CloudApiErrors::InvalidName.to_string());
    }
}
