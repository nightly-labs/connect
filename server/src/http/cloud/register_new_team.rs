use super::{
    grafana_utils::create_new_team::handle_grafana_create_new_team,
    utils::{custom_validate_name, validate_request},
};
use crate::{
    middlewares::auth_middleware::UserId, statics::TEAMS_AMOUNT_LIMIT_PER_USER,
    structs::cloud::api_cloud_errors::CloudApiErrors,
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
use uuid7::uuid7;

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
        Ok(Some(_)) => {
            return Err((
                StatusCode::BAD_REQUEST,
                CloudApiErrors::TeamAlreadyExists.to_string(),
            ));
        }
        Ok(None) => {
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

            // Grafana, add new team
            let grafana_team_id =
                handle_grafana_create_new_team(&grafana_conf, &admin_email, &user_id).await?;

            // Create a new team
            let team_id = uuid7().to_string();
            let team = Team {
                team_id: grafana_team_id.to_string(),
                team_name: request.team_name.clone(),
                team_admin_id: user_id.clone(),
                subscription: None,
                personal: request.personal,
                registration_timestamp: get_current_datetime(),
            };

            if let Err(err) = db.create_new_team(&team).await {
                error!("Failed to create team {:?}", err);
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    CloudApiErrors::DatabaseError.to_string(),
                ));
            }

            return Ok(Json(HttpRegisterNewTeamResponse { team_id }));
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

#[cfg(feature = "cloud_db_tests")]
#[cfg(test)]
mod tests {
    use crate::{
        env::{GRAFANA_API_KEY, GRAFANA_BASE_PATH, JWT_SECRET},
        http::cloud::register_new_team::{HttpRegisterNewTeamRequest, HttpRegisterNewTeamResponse},
        statics::DASHBOARD_TEMPLATE_UID,
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
    use openapi::{
        apis::{
            configuration::{ApiKey, Configuration},
            dashboards_api::{get_dashboard_by_uid, import_dashboard},
            folder_permissions_api::update_folder_permissions,
            folders_api::create_folder,
            teams_api::create_team,
        },
        models::{
            CreateFolderCommand, CreateTeamCommand, DashboardAclUpdateItem, ImportDashboardRequest,
            UpdateDashboardAclCommand,
        },
    };
    use serde_json::json;
    use std::{net::SocketAddr, sync::Arc};
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
    async fn test_grafana_create_team() {
        let team_name = "test_team_name".to_string();
        let email = "test69@gmail.com".to_string();

        let grafana_team_name = format!("[{}][{}]", team_name, email);
        // Grafana, add new team
        let grafana_request = CreateTeamCommand {
            email: Some(email.to_string()),
            name: Some(grafana_team_name.to_string()),
        };

        let mut conf = Configuration::new();
        conf.base_path = GRAFANA_BASE_PATH().to_string();
        conf.api_key = Some(ApiKey {
            prefix: Some("Bearer".to_string()),
            key: GRAFANA_API_KEY().to_string(),
        });

        let grafana_client_conf = Arc::new(conf);

        // Send request and return team id
        let grafana_team_id = match create_team(&grafana_client_conf, grafana_request).await {
            Ok(response) => match response.team_id {
                Some(team_id) => {
                    println!("Team id: {}", team_id);
                    team_id
                }
                None => {
                    panic!("Failed to create team: {:?}", response);
                }
            },
            Err(err) => {
                panic!("Failed to create team: {:?}", err);
            }
        };

        // Send request and return team id
        // Grafana, create folder
        let grafana_request = CreateFolderCommand {
            description: None,
            parent_uid: None,
            title: Some(grafana_team_name),
            uid: Some(grafana_team_id.to_string()),
        };

        let folder_uid = match create_folder(&grafana_client_conf, grafana_request).await {
            Ok(response) => {
                println!("Folder created: {:?}", response);
                response.uid.unwrap()
            }
            Err(err) => {
                panic!("Failed to create folder: {:?}", err);
            }
        };

        println!("Team id created: {}", grafana_team_id);
        println!("Folder uid: {}", folder_uid);
    }

    #[tokio::test]
    async fn test_grafana_permissions() {
        let grafana_team_id = 36;

        let mut conf = Configuration::new();
        conf.base_path = GRAFANA_BASE_PATH().to_string();
        conf.api_key = Some(ApiKey {
            prefix: Some("Bearer".to_string()),
            key: GRAFANA_API_KEY().to_string(),
        });

        let grafana_client_conf = Arc::new(conf);

        // set folder permissions for the whole team
        let update_permissions_request = UpdateDashboardAclCommand {
            items: Some(vec![DashboardAclUpdateItem {
                permission: Some(1), // Grant View permission for the whole team
                role: None,
                team_id: Some(grafana_team_id),
                user_id: None,
            }]),
        };

        match update_folder_permissions(
            &grafana_client_conf,
            &grafana_team_id.to_string(),
            update_permissions_request,
        )
        .await
        {
            Ok(response) => {
                println!("Permissions updated: {:?}", response);
            }
            Err(err) => {
                panic!("Failed to update permissions: {:?}", err);
            }
        };
    }

    #[tokio::test]
    async fn test_grafana_import_dashboard() {
        let grafana_team_id = 36;
        let app_name = "test_app_name".to_string();
        let app_id = "test_app_id".to_string();

        let mut conf = Configuration::new();
        conf.base_path = GRAFANA_BASE_PATH().to_string();
        conf.api_key = Some(ApiKey {
            prefix: Some("Bearer".to_string()),
            key: GRAFANA_API_KEY().to_string(),
        });

        let grafana_client_conf = Arc::new(conf);

        let mut dashboard =
            match get_dashboard_by_uid(&grafana_client_conf, &DASHBOARD_TEMPLATE_UID).await {
                Ok(response) => response.dashboard.unwrap(),
                Err(err) => {
                    panic!("Failed to create folder: {:?}", err);
                }
            };

        println!("DASHBOARD: {:#?}", dashboard.get("uid"));
        println!("DASHBOARD: {:#?}", dashboard.get("id"));
        println!("DASHBOARD: {:#?}", dashboard.get("title"));

        let dashboard_as_map = dashboard.as_object_mut().unwrap();

        *dashboard_as_map.get_mut("uid").unwrap() = json!(app_id);
        *dashboard_as_map.get_mut("id").unwrap() = json!("");
        *dashboard_as_map.get_mut("title").unwrap() = json!(app_name);

        println!(
            "--------------------\nDASHBOARD: {:#?}",
            dashboard.get("uid")
        );
        println!("DASHBOARD: {:#?}", dashboard.get("id"));
        println!("DASHBOARD: {:#?}", dashboard.get("title"));

        // Import dashboard
        match import_dashboard(
            &grafana_client_conf,
            ImportDashboardRequest {
                dashboard: Some(dashboard),
                folder_id: None,
                folder_uid: Some(grafana_team_id.to_string()),
                inputs: None,
                overwrite: Some(false),
                path: None,
                plugin_id: None,
            },
        )
        .await
        {
            Ok(response) => println!("DASHBOARD import: {:#?}", response),
            Err(err) => {
                panic!("Failed to create folder: {:?}", err);
            }
        };
    }

    #[tokio::test]
    async fn test_grafana_get_dashboard() {
        let mut conf = Configuration::new();
        conf.base_path = GRAFANA_BASE_PATH().to_string();
        conf.api_key = Some(ApiKey {
            prefix: Some("Bearer".to_string()),
            key: GRAFANA_API_KEY().to_string(),
        });

        let grafana_client_conf = Arc::new(conf);

        // Send request and return team id
        let dashboard =
            match get_dashboard_by_uid(&grafana_client_conf, &DASHBOARD_TEMPLATE_UID).await {
                Ok(response) => response.dashboard.unwrap(),
                Err(err) => {
                    panic!("Failed to create folder: {:?}", err);
                }
            };

        println!("DASHBOARD: {:#?}", dashboard.get("uid"));
        println!("DASHBOARD: {:#?}", dashboard.get("title"));
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
