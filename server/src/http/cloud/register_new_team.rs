use crate::{
    auth::auth_middleware::UserId, statics::TEAMS_AMOUNT_LIMIT_PER_USER,
    structs::api_cloud_errors::CloudApiErrors, utils::custom_validate_name,
};
use axum::{extract::State, http::StatusCode, Extension, Json};
use database::{
    db::Db,
    tables::{team::table_struct::Team, utils::get_current_datetime},
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
pub struct HttpRegisterNewTeamRequest {
    #[garde(custom(custom_validate_name))]
    pub team_name: String,
    #[garde(skip)]
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
        CloudApiErrors::CloudFeatureDisabled.to_string(),
    ))?;

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
