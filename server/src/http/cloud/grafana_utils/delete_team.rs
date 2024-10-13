use crate::structs::cloud::{
    api_cloud_errors::CloudApiErrors, grafana_error::handle_grafana_error,
};
use axum::http::StatusCode;
use log::warn;
use openapi::apis::{
    configuration::Configuration,
    folders_api::delete_folder,
    teams_api::{delete_team_by_id, get_team_by_id},
};
use std::sync::Arc;

pub async fn handle_grafana_delete_team(
    grafana_conf: &Arc<Configuration>,
    team_id: &String,
    app_ids: &Vec<String>,
) -> Result<(), (StatusCode, String)> {
    match get_team_by_id(&grafana_conf, &team_id).await {
        Ok(response) => match response.id {
            Some(_) => (),
            None => {
                warn!("Failed to get team: {:?}, team_id: {:?}", response, team_id);
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    CloudApiErrors::TeamDoesNotExist.to_string(),
                ));
            }
        },
        Err(err) => {
            warn!("Failed to get team: {:?}, team_id: {:?}", err, team_id);
            return Err(handle_grafana_error(err));
        }
    };

    match delete_team_by_id(&grafana_conf, team_id).await {
        Ok(_) => (),
        Err(err) => {
            warn!("Failed to delete team: {:?}, team_id: {:?}", err, team_id);
            return Err(handle_grafana_error(err));
        }
    }

    let _: () = match delete_folder(&grafana_conf, team_id, None).await {
        Ok(_) => {}
        Err(err) => {
            return Err(handle_grafana_error(err));
        }
    };
    return Ok(());
}
