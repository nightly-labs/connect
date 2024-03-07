use crate::{
    auth::auth_middleware::UserId, statics::USERS_AMOUNT_LIMIT_PER_TEAM,
    structs::api_cloud_errors::CloudApiErrors, utils::custom_validate_uuid,
};
use axum::{extract::State, http::StatusCode, Extension, Json};
use database::db::Db;
use garde::Validate;
use log::error;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, sync::Arc};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS, Validate)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpAddUserToTeamRequest {
    #[garde(custom(custom_validate_uuid))]
    pub team_id: String,
    #[garde(email)]
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
        CloudApiErrors::CloudFeatureDisabled.to_string(),
    ))?;

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

            // Check if team has at least one registered app
            match db.get_registered_apps_by_team_id(&request.team_id).await {
                Ok(apps) => {
                    if apps.is_empty() {
                        return Err((
                            StatusCode::BAD_REQUEST,
                            CloudApiErrors::TeamHasNoRegisteredApps.to_string(),
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
                            CloudApiErrors::MaximumUsersPerTeamReached.to_string(),
                        ));
                    }
                }
                Err(err) => {
                    error!("Failed to get privileges by team id: {:?}", err);
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        CloudApiErrors::DatabaseError.to_string(),
                    ));
                }
            }

            // Get user data and perform checks
            let user = match db.get_user_by_email(&request.user_email).await {
                Ok(Some(user)) => user,
                Ok(None) => {
                    return Err((
                        StatusCode::BAD_REQUEST,
                        CloudApiErrors::UserDoesNotExist.to_string(),
                    ));
                }
                Err(err) => {
                    error!("Failed to get user by email: {:?}", err);
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        CloudApiErrors::DatabaseError.to_string(),
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
                            CloudApiErrors::UserAlreadyBelongsToTheTeam.to_string(),
                        ));
                    }
                }
                Err(err) => {
                    error!(
                        "Failed to get teams and apps membership by user id: {:?}",
                        err
                    );
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        CloudApiErrors::DatabaseError.to_string(),
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
                Err(err) => {
                    error!("Failed to add user to the team: {:?}", err);
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        CloudApiErrors::DatabaseError.to_string(),
                    ));
                }
            }
        }
        Ok(None) => {
            return Err((StatusCode::BAD_REQUEST, "Team does not exist".to_string()));
        }
        Err(err) => {
            error!("Failed to get team: {:?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::DatabaseError.to_string(),
            ));
        }
    }
}
