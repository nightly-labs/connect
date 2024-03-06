use crate::{auth::auth_middleware::UserId, utils::TEAMS_AMOUNT_LIMIT_PER_USER};
use axum::{extract::State, http::StatusCode, Extension, Json};
use database::{
    db::Db,
    tables::{team::table_struct::Team, utils::get_current_datetime},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ts_rs::TS;
use uuid7::uuid7;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpRegisterNewTeamRequest {
    pub team_name: String,
    pub personal: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct HttpRegisterNewTeamResponse {
    pub team_id: String,
}

pub async fn register_new_team(
    State(db): State<Option<Arc<Db>>>,
    Extension(user_id): Extension<UserId>,
    Json(request): Json<HttpRegisterNewTeamRequest>,
) -> Result<Json<HttpRegisterNewTeamResponse>, (StatusCode, String)> {
    // Db connection has already been checked in the middleware
    let db = db.as_ref().ok_or((
        StatusCode::INTERNAL_SERVER_ERROR,
        "Failed to get database connection".to_string(),
    ))?;

    // First check if user is creating a new team
    // Get team data and perform checks
    match db
        .get_team_by_team_name_and_admin_id(&request.team_name, &user_id)
        .await
    {
        Ok(Some(_)) => {
            return Err((StatusCode::BAD_REQUEST, "Team already exists".to_string()));
        }
        Ok(None) => {
            // Check how many teams the user has
            match db.get_admin_user_teams_without_personal(&user_id).await {
                Ok(teams) => {
                    if teams.len() >= TEAMS_AMOUNT_LIMIT_PER_USER {
                        return Err((
                            StatusCode::BAD_REQUEST,
                            "User has reached the maximum number of teams".to_string(),
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

            // Check if user already has a personal team
            if request.personal {
                match db.get_personal_team_by_admin_id(&user_id).await {
                    Ok(Some(_)) => {
                        return Err((
                            StatusCode::BAD_REQUEST,
                            "User already has a personal team".to_string(),
                        ));
                    }
                    Ok(None) => {
                        // Continue
                    }
                    Err(_) => {
                        return Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            "Database error".to_string(),
                        ));
                    }
                }
            }

            // Create a new team
            let team_id = uuid7().to_string();
            let team = Team {
                team_id: team_id.clone(),
                team_name: request.team_name.clone(),
                team_admin_id: user_id.clone(),
                subscription: None,
                personal: request.personal,
                registration_timestamp: get_current_datetime(),
            };

            if let Err(_) = db.create_new_team(&team).await {
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Failed to create a new team".to_string(),
                ));
            }

            return Ok(Json(HttpRegisterNewTeamResponse { team_id }));
        }
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error".to_string(),
            ));
        }
    }
}
