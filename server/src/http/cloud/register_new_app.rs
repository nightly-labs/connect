use crate::{
    middlewares::auth_middleware::UserId,
    statics::REGISTERED_APPS_LIMIT_PER_TEAM,
    structs::cloud::api_cloud_errors::CloudApiErrors,
    utils::{custom_validate_name, custom_validate_uuid, validate_request},
};
use axum::{extract::State, http::StatusCode, Extension, Json};
use database::{
    db::Db, structs::privilege_level::PrivilegeLevel, tables::utils::get_current_datetime,
};
use garde::Validate;
use log::error;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ts_rs::TS;
use uuid7::uuid7;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS, Validate)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpRegisterNewAppRequest {
    #[garde(custom(custom_validate_uuid))]
    pub team_id: String,
    #[garde(custom(custom_validate_name))]
    pub app_name: String,
    #[garde(skip)]
    pub whitelisted_domains: Vec<String>,
    #[garde(skip)]
    pub ack_public_keys: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpRegisterNewAppResponse {
    pub app_id: String,
}

pub async fn register_new_app(
    State(db): State<Option<Arc<Db>>>,
    Extension(user_id): Extension<UserId>,
    Json(request): Json<HttpRegisterNewAppRequest>,
) -> Result<Json<HttpRegisterNewAppResponse>, (StatusCode, String)> {
    // Db connection has already been checked in the middleware
    let db = db.as_ref().ok_or((
        StatusCode::INTERNAL_SERVER_ERROR,
        CloudApiErrors::CloudFeatureDisabled.to_string(),
    ))?;

    // Validate request
    validate_request(&request, &())?;

    // First check if user is adding a new app to an existing team
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

            // Check if user has already registered an app with this name in this team
            match db
                .get_registered_app_by_app_name_and_team_id(&request.app_name, &request.team_id)
                .await
            {
                Ok(Some(_)) => {
                    return Err((
                        StatusCode::BAD_REQUEST,
                        CloudApiErrors::AppAlreadyExists.to_string(),
                    ));
                }
                Ok(None) => {}
                Err(err) => {
                    error!(
                        "Failed to get registered app by app name and team id: {:?}",
                        err
                    );
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        CloudApiErrors::DatabaseError.to_string(),
                    ));
                }
            }

            // Check how many apps the team has
            match db.get_registered_apps_by_team_id(&request.team_id).await {
                Ok(apps) => {
                    if apps.len() >= REGISTERED_APPS_LIMIT_PER_TEAM {
                        return Err((
                            StatusCode::BAD_REQUEST,
                            CloudApiErrors::MaximumAppsPerTeamReached.to_string(),
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

            // Register a new app under this team
            // Start a transaction
            let mut tx = db.connection_pool.begin().await.unwrap();

            // Register a new app
            let app_id = uuid7().to_string();
            let db_registered_app =
                database::tables::registered_app::table_struct::DbRegisteredApp {
                    app_id: app_id.clone(),
                    team_id: team.team_id.clone(),
                    app_name: request.app_name.clone(),
                    ack_public_keys: request.ack_public_keys.clone(),
                    whitelisted_domains: request.whitelisted_domains.clone(),
                    registration_timestamp: get_current_datetime(),
                };

            if let Err(err) = db
                .register_new_app_within_tx(&mut tx, &db_registered_app)
                .await
            {
                let _ = tx
                    .rollback()
                    .await
                    .map_err(|err| error!("Failed to rollback transaction: {:?}", err));
                error!("Failed to create app: {:?}", err);
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    CloudApiErrors::DatabaseError.to_string(),
                ));
            }

            // Create a new user app privilege
            let user_app_privilege =
                database::tables::user_app_privileges::table_struct::UserAppPrivilege {
                    app_id: app_id.clone(),
                    creation_timestamp: get_current_datetime(),
                    privilege_level: PrivilegeLevel::Admin,
                    user_id: user_id.clone(),
                };

            if let Err(err) = db
                .add_new_privilege_within_tx(&mut tx, &user_app_privilege)
                .await
            {
                let _ = tx
                    .rollback()
                    .await
                    .map_err(|err| error!("Failed to rollback transaction: {:?}", err));
                error!("Failed to create user app privilege {:?}", err);
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    CloudApiErrors::DatabaseError.to_string(),
                ));
            }

            // Add read access to the app for the existing users
            if let Err(err) = db
                .add_privileges_for_new_team_app_for_existing_users(
                    &mut tx,
                    &team.team_id,
                    &team.team_admin_id,
                    &app_id,
                )
                .await
            {
                let _ = tx
                    .rollback()
                    .await
                    .map_err(|err| error!("Failed to rollback transaction: {:?}", err));
                error!("Failed to add read privileges to existing users: {:?}", err);
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    CloudApiErrors::DatabaseError.to_string(),
                ));
            }

            // If nothing failed commit the transaction
            tx.commit().await.unwrap();
            return Ok(Json(HttpRegisterNewAppResponse { app_id }));
        }
        Ok(None) => {
            return Err((
                StatusCode::BAD_REQUEST,
                CloudApiErrors::TeamDoesNotExist.to_string(),
            ));
        }
        Err(err) => {
            error!("Failed to get team by team id: {:?}", err);
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
        http::cloud::register_new_app::{HttpRegisterNewAppRequest, HttpRegisterNewAppResponse},
        structs::cloud::{
            api_cloud_errors::CloudApiErrors, cloud_http_endpoints::HttpCloudEndpoint,
        },
        test_utils::test_utils::{
            add_test_team, convert_response, create_test_app, generate_valid_name,
            register_and_login_random_user,
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
    async fn test_register_new_app() {
        let test_app = create_test_app(false).await;

        let (auth_token, _email, _password) = register_and_login_random_user(&test_app).await;

        // Register new team
        let team_name = generate_valid_name();
        let team_id = add_test_team(&team_name, &auth_token, &test_app, false)
            .await
            .unwrap();

        // Register new app
        let app_name = generate_valid_name();
        let request = HttpRegisterNewAppRequest {
            team_id: team_id.clone(),
            app_name: app_name.clone(),
            whitelisted_domains: vec![],
            ack_public_keys: vec![],
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
                HttpCloudEndpoint::RegisterNewApp.to_string()
            ))
            .extension(ip)
            .body(Body::from(json))
            .unwrap();

        // Send request
        let response = test_app.clone().oneshot(req).await.unwrap();
        // Validate response
        let _ = convert_response::<HttpRegisterNewAppResponse>(response)
            .await
            .unwrap();

        // Try to register the new app again, should fail
        let request = HttpRegisterNewAppRequest {
            team_id: team_id.clone(),
            app_name: app_name.clone(),
            whitelisted_domains: vec![],
            ack_public_keys: vec![],
        };

        let json = serde_json::to_string(&request).unwrap();
        let req = Request::builder()
            .method(Method::POST)
            .header("content-type", "application/json")
            .header("authorization", format!("Bearer {auth}"))
            .uri(format!(
                "/cloud/private{}",
                HttpCloudEndpoint::RegisterNewApp.to_string()
            ))
            .extension(ip)
            .body(Body::from(json))
            .unwrap();

        // Send request
        let response = test_app.clone().oneshot(req).await.unwrap();
        // Validate response
        let err = convert_response::<HttpRegisterNewAppResponse>(response)
            .await
            .unwrap_err();

        assert_eq!(
            err.to_string(),
            CloudApiErrors::AppAlreadyExists.to_string()
        );
    }

    #[tokio::test]
    async fn test_invalid_app_name() {
        let test_app = create_test_app(false).await;

        let (auth_token, _email, _password) = register_and_login_random_user(&test_app).await;

        // Register new team
        let team_name = generate_valid_name();
        let team_id = add_test_team(&team_name, &auth_token, &test_app, false)
            .await
            .unwrap();

        // Register new app
        let app_name = generate_valid_name() + "18702dhb12n1902hd89b1n28sd1 02n>>>>>>>>>>>>>>>";
        let request = HttpRegisterNewAppRequest {
            team_id: team_id.clone(),
            app_name: app_name.clone(),
            whitelisted_domains: vec![],
            ack_public_keys: vec![],
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
                HttpCloudEndpoint::RegisterNewApp.to_string()
            ))
            .extension(ip)
            .body(Body::from(json))
            .unwrap();

        // Send request
        let response = test_app.clone().oneshot(req).await.unwrap();
        // Validate response
        let err = convert_response::<HttpRegisterNewAppResponse>(response)
            .await
            .unwrap_err();

        assert_eq!(err.to_string(), CloudApiErrors::InvalidName.to_string());
    }
}
