use axum::{extract::State, http::StatusCode, Json};
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
use ts_rs::TS;
use uuid7::uuid7;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpRegisterNewAppRequest {
    pub team_id: Option<String>,
    pub user_id: String,
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
    State(db): State<Db>,
    Json(request): Json<HttpRegisterNewAppRequest>,
) -> Result<Json<HttpRegisterNewAppResponse>, (StatusCode, String)> {
    // Check if app is already registered
    if let Ok(_) = db.get_registered_app_by_app_name(&request.app_name).await {
        return Err((
            StatusCode::BAD_REQUEST,
            "App with this name already exists".to_string(),
        ));
    }

    // Check if we are creating a new team or not
    match request.team_id {
        Some(team_id) => {
            // Start a transaction
            let mut tx = db.connection_pool.begin().await.unwrap();

            let team = db.get_team_by_team_id(Some(&mut tx), &team_id).await;

            match team {
                Ok(Some(team)) => {
                    // If team_id is provided check if user is a admin of the team
                    if team.team_admin_id != request.user_id {
                        return Err((
                            StatusCode::BAD_REQUEST,
                            "User does not have permissions to create a new app for this team"
                                .to_string(),
                        ));
                    }

                    // Register a new app
                    let app_id = uuid7().to_string();
                    let db_registered_app =
                        database::tables::registered_app::table_struct::DbRegisteredApp {
                            app_id: app_id.clone(),
                            team_id: team_id.clone(),
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
                    return Err((StatusCode::BAD_REQUEST, "Team does not exist".to_string()));
                }
                Err(err) => {
                    tx.rollback().await.unwrap();
                    error!("Failed to create app: {:?}", err);
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Failed to create app".to_string(),
                    ));
                }
            }
        }
        None => {
            // Check if user is already a admin of a team
            if let Ok(privileges) = db.get_privileges_by_user_id(&request.user_id).await {
                let is_already_admin = privileges.iter().any(|privilege| {
                    if let PrivilegeLevel::Admin = privilege.privilege_level {
                        return true;
                    }
                    return false;
                });

                if is_already_admin {
                    return Err((
                        StatusCode::BAD_REQUEST,
                        "User is already an admin of a team".to_string(),
                    ));
                }
            }

            // Register a new app under a new team
            let team_id = uuid7().to_string();
            let app_id = uuid7().to_string();
            let time = get_current_datetime();

            let team = Team {
                team_id: team_id.clone(),
                subscription: None,
                team_admin_id: request.user_id.clone(),
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
                user_id: request.user_id.clone(),
            };

            db.setup_team(&team, &db_registered_app, &user_app_privilege)
                .await
                .unwrap();

            return Ok(Json(HttpRegisterNewAppResponse { app_id }));
        }
    }
}
