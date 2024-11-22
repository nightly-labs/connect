use crate::structs::cloud::{
    api_cloud_errors::CloudApiErrors, grafana_error::handle_grafana_error,
};
use axum::http::StatusCode;
use log::{error, warn};
use openapi::{
    apis::{
        configuration::Configuration,
        teams_api::add_team_member,
        users_api::{get_user_by_login_or_email, get_user_teams},
    },
    models::AddTeamMemberCommand,
};
use std::sync::Arc;

pub async fn handle_grafana_add_user_to_team(
    grafana_conf: &Arc<Configuration>,
    team_id: &String,
    user_email: &String,
) -> Result<(), (StatusCode, String)> {
    // Check if user exists, if not create a new user
    let user_id = match get_user_by_login_or_email(&grafana_conf, user_email).await {
        Ok(user) => user.id,
        Err(err) => {
            warn!("Failed to get user from grafana: {:?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::InternalServerError.to_string(),
            ));
        }
    };

    // If for some reason user_id is not found, return error
    let id = match user_id {
        Some(id) => id,
        None => {
            error!("Failed to get user_id for email: {:?}", user_email);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::InternalServerError.to_string(),
            ));
        }
    };

    // Check if user is already in the team
    match get_user_teams(&grafana_conf, id.clone()).await {
        Ok(teams) => {
            let team_id: i64 = team_id.parse().map_err(|err| {
                error!("Failed to parse team_id: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    CloudApiErrors::InternalServerError.to_string(),
                )
            })?;

            // For now we will be checking team id but in the future we might need to swap to team uid
            if teams.iter().any(|team| team.id == Some(team_id)) {
                return Err((
                    StatusCode::BAD_REQUEST,
                    CloudApiErrors::UserAlreadyBelongsToTheTeam.to_string(),
                ));
            }
        }
        Err(err) => {
            warn!("Failed to get user teams: {:?}", err);
            return Err(handle_grafana_error(err));
        }
    }

    // Add user to the team
    let request = AddTeamMemberCommand { user_id: user_id };

    if let Err(err) = add_team_member(&grafana_conf, team_id, request).await {
        warn!(
            "Failed to add user [{:?}] to team [{:?}], error: {:?}",
            user_email, team_id, err
        );
        return Err(handle_grafana_error(err));
    }

    Ok(())
}
