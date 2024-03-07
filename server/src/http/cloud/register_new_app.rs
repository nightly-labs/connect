use crate::{auth::auth_middleware::UserId, statics::REGISTERED_APPS_LIMIT_PER_TEAM};
use axum::{extract::State, http::StatusCode, Extension, Json};
use database::{
    db::Db, structs::privilege_level::PrivilegeLevel, tables::utils::get_current_datetime,
};
use log::error;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ts_rs::TS;
use uuid7::uuid7;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpRegisterNewAppRequest {
    pub team_id: String,
    pub app_name: String,
    pub whitelisted_domains: Vec<String>,
    pub ack_public_keys: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
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
        "Failed to get database connection".to_string(),
    ))?;

    // First check if user is adding a new app to an existing team
    // Get team data and perform checks
    match db.get_team_by_team_id(None, &request.team_id).await {
        Ok(Some(team)) => {
            // Check if user is a admin of this team
            if team.team_admin_id != user_id {
                return Err((
                    StatusCode::BAD_REQUEST,
                    "Insufficient permissions to register new app".to_string(),
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
                        "App with this name already exists".to_string(),
                    ));
                }
                Ok(None) => {}
                Err(_) => {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Database error".to_string(),
                    ));
                }
            }

            // Check how many apps the team has
            match db.get_registered_apps_by_team_id(&request.team_id).await {
                Ok(apps) => {
                    if apps.len() >= REGISTERED_APPS_LIMIT_PER_TEAM {
                        return Err((
                            StatusCode::BAD_REQUEST,
                            "Team has reached the maximum number of apps".to_string(),
                        ));
                    }
                }
                Err(_) => {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Database error".to_string(),
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
                    email: None,
                    pass_hash: None,
                    registration_timestamp: get_current_datetime(),
                    subscription: None,
                };

            if let Err(err) = db
                .register_new_app_within_tx(&mut tx, &db_registered_app)
                .await
            {
                tx.rollback().await.unwrap();
                error!("Failed to create app: {:?}", err);
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to create app".to_string(),
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

            if let Err(_) = db
                .add_new_privilege_within_tx(&mut tx, &user_app_privilege)
                .await
            {
                tx.rollback().await.unwrap();
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to create user app privilege".to_string(),
                ));
            }

            // Add read access to the app for the existing users
            if let Err(_) = db
                .add_privileges_for_new_team_app_for_existing_users(
                    &mut tx,
                    &team.team_id,
                    &team.team_admin_id,
                    &app_id,
                )
                .await
            {
                tx.rollback().await.unwrap();
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to add read privileges to existing users".to_string(),
                ));
            }

            // If nothing failed commit the transaction
            tx.commit().await.unwrap();
            return Ok(Json(HttpRegisterNewAppResponse { app_id }));
        }
        Ok(None) => {
            return Err((StatusCode::BAD_REQUEST, "Team does not exist".to_string()));
        }
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error".to_string(),
            ));
        }
    }
}
