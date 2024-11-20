use super::utils::{custom_validate_uuid, validate_request};
use crate::{
    env::is_env_production,
    http::cloud::grafana_utils::delete_team::handle_grafana_delete_team,
    middlewares::auth_middleware::UserId,
    structs::cloud::{api_cloud_errors::CloudApiErrors, app_info::AppInfo},
};
use axum::{extract::State, http::StatusCode, Extension, Json};
use database::db::Db;
use garde::Validate;
use log::{error, warn};
use openapi::apis::configuration::Configuration;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS, Validate)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpDeleteTeamRequest {
    #[garde(custom(custom_validate_uuid))]
    pub team_id: String,
}

pub async fn delete_team(
    State(db): State<Arc<Db>>,
    State(grafana_conf): State<Arc<Configuration>>,
    Extension(user_id): Extension<UserId>,
    Json(request): Json<HttpDeleteTeamRequest>,
) -> Result<Json<()>, (StatusCode, String)> {
    // Validate request
    validate_request(&request, &())?;
    warn!("Delete team request: {:?}", request);
    // Start a transaction
    let mut tx = db.connection_pool.begin().await.unwrap();

    // First check if team exists
    let team = match db.get_team_by_team_id(None, &request.team_id).await {
        Ok(Some(team)) => {
            if team.team_admin_id != user_id {
                return Err((
                    StatusCode::BAD_REQUEST,
                    CloudApiErrors::InsufficientPermissions.to_string(),
                ));
            }

            // Check if team is active
            if team.deactivated_at != None {
                return Err((
                    StatusCode::BAD_REQUEST,
                    CloudApiErrors::TeamDoesNotExist.to_string(),
                ));
            }
            team
        }
        Ok(None) => {
            return Err((
                StatusCode::BAD_REQUEST,
                CloudApiErrors::TeamDoesNotExist.to_string(),
            ));
        }
        Err(err) => {
            error!("Failed to get app by app id: {:?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::DatabaseError.to_string(),
            ));
        }
    };

    // Delete the team
    if let Err(err) = db.deactivate_team(&mut tx, &request.team_id).await {
        let _ = tx
            .rollback()
            .await
            .map_err(|err| error!("Failed to rollback transaction: {:?}", err));
        error!("Failed to deactivate team: {:?}", err);
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            CloudApiErrors::DatabaseError.to_string(),
        ));
    }

    // Delete all team invites
    if let Err(err) = db.cancel_all_team_invites(&mut tx, &request.team_id).await {
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

    // Get team apps
    let registered_apps: Vec<AppInfo> = match db.get_registered_apps_by_team_id(&team.team_id).await
    {
        Ok(apps) => apps.into_iter().map(|app| app.into()).collect(),
        Err(err) => {
            error!("Failed to get registered apps by team_id: {:?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::DatabaseError.to_string(),
            ));
        }
    };

    // Delete team apps, privileges and domain verifications
    for app in registered_apps.iter() {
        if let Err(err) = db.deactivate_app(&mut tx, &app.app_id).await {
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
            .remove_privileges_for_inactive_app_within_tx(&mut tx, &app.app_id)
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
        if let Err(err) = db
            .delete_domain_verification_for_inactive_app(&mut tx, &app.app_id)
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
    }

    // Grafana, delete team
    // TODO, fix this by fixing methods for setting up grafana datasource
    if is_env_production() {
        let team_grafana_id = match team.grafana_id {
            Some(grafana_id) => grafana_id,
            None => {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    CloudApiErrors::TeamWithoutGrafanaId.to_string(),
                ));
            }
        };

        if let Err(err) = handle_grafana_delete_team(&grafana_conf, &team_grafana_id).await {
            error!("Failed to delete team from grafana: {:?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::GrafanaError.to_string(),
            ));
        };
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

#[cfg(feature = "cloud_integration_tests")]
#[cfg(test)]
mod tests {
    use crate::{
        env::JWT_SECRET,
        http::cloud::{
            delete_team::HttpDeleteTeamRequest, register_new_app::HttpRegisterNewAppRequest,
        },
        structs::cloud::cloud_http_endpoints::HttpCloudEndpoint,
        test_utils::test_utils::{
            add_test_app, add_test_team, add_user_to_test_team, convert_response, create_test_app,
            generate_valid_name, get_test_team_data, register_and_login_random_user,
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
    async fn test_delete_team() {
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
        add_test_app(&request, &auth_token, &test_app)
            .await
            .unwrap();
        let _ =
            add_user_to_test_team(&team_id, &user_email, &auth_token, &user_token, &test_app).await;

        let request = HttpDeleteTeamRequest {
            team_id: team_id.clone(),
        };
        let ip: ConnectInfo<SocketAddr> = ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 8080)));
        let json = serde_json::to_string(&request).unwrap();
        let auth = auth_token.encode(JWT_SECRET()).unwrap();

        let team = get_test_team_data(&team_id, &auth_token, &test_app)
            .await
            .unwrap();
        assert_eq!(team.team_metadata.team_id, team_id.clone());

        let req = Request::builder()
            .method(Method::POST)
            .header("content-type", "application/json")
            .header("authorization", format!("Bearer {auth}"))
            .uri(format!(
                "/cloud/private{}",
                HttpCloudEndpoint::DeleteTeam.to_string()
            ))
            .extension(ip)
            .body(Body::from(json))
            .unwrap();

        // Send request
        let response = test_app.clone().oneshot(req).await.unwrap();
        let _ = convert_response::<()>(response).await.unwrap();
        let err = get_test_team_data(&team_id, &auth_token, &test_app)
            .await
            .unwrap_err();

        assert_eq!(err.to_string(), "TeamDoesNotExist".to_string());
    }
}
