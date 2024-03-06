use crate::auth::auth_middleware::UserId;
use axum::{extract::State, http::StatusCode, Extension, Json};
use database::{
    db::Db,
    structs::privelage_level::PrivilegeLevel,
    tables::{
        registered_app::table_struct::DbRegisteredApp, team::table_struct::Team,
        user_app_privileges::table_struct::UserAppPrivilege, utils::get_current_datetime,
    },
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

    // Check if app is already registered
    if let Ok(_) = db.get_registered_app_by_app_name(&request.app_name).await {
        return Err((
            StatusCode::BAD_REQUEST,
            "App with this name already exists".to_string(),
        ));
    }

    // check if user has created a team before
    match db.get_team_by_admin_id(&user_id).await {
        Ok(Some(team)) => {
            // User is a admin of a team, register a new app under this team
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

            tx.commit().await.unwrap();
            return Ok(Json(HttpRegisterNewAppResponse { app_id }));
        }
        Ok(None) => {
            // User us not a admin of a team, create a new team and register a new app under this team
            let team_id = uuid7().to_string();
            let app_id = uuid7().to_string();
            let time = get_current_datetime();

            let team = Team {
                team_id: team_id.clone(),
                subscription: None,
                team_admin_id: user_id.clone(),
                registration_timestamp: time.clone(),
            };

            let db_registered_app = DbRegisteredApp {
                app_id: app_id.clone(),
                team_id: team_id.clone(),
                app_name: request.app_name.clone(),
                ack_public_keys: request.ack_public_keys.clone(),
                whitelisted_domains: request.whitelisted_domains.clone(),
                email: None,
                pass_hash: None,
                registration_timestamp: time.clone(),
                subscription: None,
            };

            let user_app_privilege = UserAppPrivilege {
                app_id: app_id.clone(),
                creation_timestamp: time,
                privilege_level: PrivilegeLevel::Admin,
                user_id: user_id.clone(),
            };

            if let Err(_) = db
                .setup_team(&team, &db_registered_app, &user_app_privilege)
                .await
            {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to register new app".to_string(),
                ));
            }

            return Ok(Json(HttpRegisterNewAppResponse { app_id }));
        }
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error".to_string(),
            ));
        }
    }
}
