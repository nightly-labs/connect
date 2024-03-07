use crate::{auth::auth_middleware::UserId, statics::USERS_AMOUNT_LIMIT_PER_TEAM};
use axum::{extract::State, http::StatusCode, Extension, Json};
use database::db::Db;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, sync::Arc};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpAddUserToTeamRequest {
    pub team_id: String,
    pub user_email: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct HttpAddUserToTeamResponse {}

pub async fn add_user_to_team(
    State(db): State<Option<Arc<Db>>>,
    Extension(user_id): Extension<UserId>,
    Json(request): Json<HttpAddUserToTeamRequest>,
) -> Result<Json<HttpAddUserToTeamResponse>, (StatusCode, String)> {
    // Db connection has already been checked in the middleware
    let db = db.as_ref().ok_or((
        StatusCode::INTERNAL_SERVER_ERROR,
        "Failed to get database connection".to_string(),
    ))?;

    // Get team data and perform checks
    match db.get_team_by_team_id(None, &request.team_id).await {
        Ok(Some(team)) => {
            // Check if user is a admin of this team
            if team.team_admin_id != user_id {
                return Err((
                    StatusCode::BAD_REQUEST,
                    "Insufficient permissions to add user to the team".to_string(),
                ));
            }

            // Check if team has at least one registered app
            match db.get_registered_apps_by_team_id(&request.team_id).await {
                Ok(apps) => {
                    if apps.is_empty() {
                        return Err((
                            StatusCode::BAD_REQUEST,
                            "Team has no registered apps".to_string(),
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

            // Check if limit of users in the team has been reached
            match db.get_privileges_by_team_id(&request.team_id).await {
                Ok(privileges) => {
                    let users = privileges
                        .iter()
                        .map(|privilege| privilege.user_id.clone())
                        .collect::<HashSet<String>>();

                    if users.len() >= USERS_AMOUNT_LIMIT_PER_TEAM {
                        return Err((
                            StatusCode::BAD_REQUEST,
                            "Team has reached the maximum number of users".to_string(),
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

            // Get user data and perform checks
            let user = match db.get_user_by_email(&request.user_email).await {
                Ok(Some(user)) => user,
                Ok(None) => {
                    return Err((
                        StatusCode::BAD_REQUEST,
                        "User with this email does not exist".to_string(),
                    ));
                }
                Err(_) => {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Database error".to_string(),
                    ));
                }
            };

            // Check if user already belongs to the team
            match db
                .get_teams_and_apps_membership_by_user_id(&user.user_id)
                .await
            {
                Ok(teams) => {
                    // This won't check if user has permissions to all apps in the team
                    if teams.iter().any(|(team_id, _)| team_id == &request.team_id) {
                        return Err((
                            StatusCode::BAD_REQUEST,
                            "User already belongs to the team".to_string(),
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

            // Add user to the team
            match db
                .add_user_to_the_team(&user.user_id, &request.team_id)
                .await
            {
                Ok(_) => {
                    return Ok(Json(HttpAddUserToTeamResponse {}));
                }
                Err(_) => {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Database error".to_string(),
                    ));
                }
            }
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
