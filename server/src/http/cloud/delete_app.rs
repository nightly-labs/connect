use super::utils::{custom_validate_uuid, validate_request};
use crate::{
    env::is_env_production,
    http::cloud::grafana_utils::delete_registered_app::handle_grafana_delete_app,
    middlewares::auth_middleware::UserId, structs::cloud::api_cloud_errors::CloudApiErrors,
};
use axum::{extract::State, http::StatusCode, Extension, Json};
use database::{db::Db, structs::privilege_level::PrivilegeLevel};
use garde::Validate;
use log::{error, warn};
use openapi::apis::configuration::Configuration;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS, Validate)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpDeleteAppRequest {
    #[garde(custom(custom_validate_uuid))]
    pub app_id: String,
}

pub async fn delete_app(
    State(db): State<Arc<Db>>,
    State(grafana_conf): State<Arc<Configuration>>,
    Extension(user_id): Extension<UserId>,
    Json(request): Json<HttpDeleteAppRequest>,
) -> Result<Json<()>, (StatusCode, String)> {
    // Validate request
    validate_request(&request, &())?;
    warn!("Delete app request: {:?}", request);

    // First check if app exists
    match db.get_registered_app_by_app_id(&request.app_id).await {
        Ok(Some(app)) => {
            // Check if user has admin privileges
            match db
                .get_privilege_by_user_id_and_app_id(&user_id, &request.app_id)
                .await
            {
                Ok(Some(user_privilege)) => {
                    if user_privilege.privilege_level != PrivilegeLevel::Admin {
                        return Err((
                            StatusCode::BAD_REQUEST,
                            CloudApiErrors::InsufficientPermissions.to_string(),
                        ));
                    }
                    // Check if app is active
                    if app.deactivated_at != None {
                        return Err((
                            StatusCode::BAD_REQUEST,
                            CloudApiErrors::AppDoesNotExist.to_string(),
                        ));
                    }
                }
                Ok(None) => {
                    return Err((
                        StatusCode::BAD_REQUEST,
                        CloudApiErrors::InsufficientPermissions.to_string(),
                    ));
                }
                Err(err) => {
                    error!("Failed to get privileges by app id and user id: {:?}", err);
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        CloudApiErrors::DatabaseError.to_string(),
                    ));
                }
            }
            // Delete the app
            // Start a transaction
            let mut tx = db.connection_pool.begin().await.unwrap();

            if let Err(err) = db.deactivate_app(&mut tx, &request.app_id).await {
                let _ = tx
                    .rollback()
                    .await
                    .map_err(|err| error!("Failed to rollback transaction: {:?}", err));
                error!("Failed to deactivate app: {:?}", err);
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    CloudApiErrors::DatabaseError.to_string(),
                ));
            }
            if let Err(err) = db
                .remove_privileges_for_inactive_app_within_tx(&mut tx, &request.app_id)
                .await
            {
                let _ = tx
                    .rollback()
                    .await
                    .map_err(|err| error!("Failed to rollback transaction: {:?}", err));
                error!("Failed to delete app: {:?}", err);
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    CloudApiErrors::DatabaseError.to_string(),
                ));
            }
            if let Err(err) = db
                .delete_domain_verification_for_inactive_app(&mut tx, &request.app_id)
                .await
            {
                let _ = tx
                    .rollback()
                    .await
                    .map_err(|err| error!("Failed to rollback transaction: {:?}", err));
                error!("Failed to delete app: {:?}", err);
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    CloudApiErrors::DatabaseError.to_string(),
                ));
            }
            // Grafana, delete app
            // TODO, fix this by fixing methods for setting up grafana datasource
            if is_env_production() {
                match handle_grafana_delete_app(&grafana_conf, &request.app_id).await {
                    Ok(_) => {}
                    Err(err) => {
                        error!("Failed to delete app from grafana: {:?}", err);
                        return Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            CloudApiErrors::GrafanaError.to_string(),
                        ));
                    }
                }
            }

            // If nothing failed commit the transaction
            if let Err(err) = tx.commit().await {
                error!("Failed to commit transaction: {:?}", err);
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    CloudApiErrors::DatabaseError.to_string(),
                ));
            }
            return Ok(Json(()));
        }
        Ok(None) => {
            return Err((
                StatusCode::BAD_REQUEST,
                CloudApiErrors::AppDoesNotExist.to_string(),
            ));
        }
        Err(err) => {
            error!("Failed to get app by app id: {:?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::DatabaseError.to_string(),
            ));
        }
    }
}

#[cfg(feature = "cloud_intsegration_tests")]
#[cfg(test)]
mod tests {
    use crate::{
        env::JWT_SECRET,
        http::cloud::{
            delete_app::HttpDeleteAppRequest, register_new_app::HttpRegisterNewAppRequest,
        },
        structs::cloud::cloud_http_endpoints::HttpCloudEndpoint,
        test_utils::test_utils::{
            add_test_app, add_test_team, add_user_to_test_team, convert_response, create_test_app,
            generate_valid_name, get_test_app_data, register_and_login_random_user,
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
    async fn test_delete_app() {
        let test_app = create_test_app(false).await;
        let (auth_token, _email, _password) = register_and_login_random_user(&test_app).await;
        let (user_token, user_email, _password) = register_and_login_random_user(&test_app).await;

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
        let app_id = add_test_app(&request, &auth_token, &test_app)
            .await
            .unwrap();

        let request = HttpRegisterNewAppRequest {
            team_id: team_id.clone(),
            app_name: generate_valid_name(),
        };
        let app_id_2 = add_test_app(&request, &auth_token, &test_app)
            .await
            .unwrap();
        let _ =
            add_user_to_test_team(&team_id, &user_email, &auth_token, &user_token, &test_app).await;

        let request = HttpDeleteAppRequest {
            app_id: app_id.clone(),
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
                HttpCloudEndpoint::DeleteApp.to_string()
            ))
            .extension(ip)
            .body(Body::from(json))
            .unwrap();

        // Send request
        let response = test_app.clone().oneshot(req).await.unwrap();
        convert_response::<()>(response).await.unwrap();
        let err = get_test_app_data(&team_id, &app_id, &auth_token, &test_app)
            .await
            .unwrap_err();

        assert_eq!(err.to_string(), "App not found".to_string());
        assert!(
            get_test_app_data(&team_id, &app_id_2, &auth_token, &test_app)
                .await
                .is_ok()
        );
    }
}
