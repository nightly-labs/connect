use crate::structs::cloud::{
    api_cloud_errors::CloudApiErrors, grafana_error::handle_grafana_error,
};
use axum::http::StatusCode;
use log::warn;
use openapi::apis::{
    admin_users_api::admin_delete_user,
    configuration::Configuration,
    folders_api::delete_folder,
    teams_api::{delete_team_by_id, get_team_by_id, remove_team_member},
    users_api::get_user_by_login_or_email,
};
use std::sync::Arc;

pub async fn handle_grafana_delete_user_account(
    grafana_conf: &Arc<Configuration>,
    owned_team_grafana_ids: &Vec<String>,
    non_owned_team_grafana_ids: &Vec<String>,
    user_email: &String,
) -> Result<(), (StatusCode, String)> {
    for team_id in owned_team_grafana_ids {
        match get_team_by_id(&grafana_conf, team_id).await {
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

        if let Err(err) = delete_folder(&grafana_conf, team_id, Some(false)).await {
            warn!("Failed to delete folder: {:?}, team_id: {:?}", err, team_id);
            return Err(handle_grafana_error(err));
        };
    }
    // Check if user exists, if not return error
    let user_id = match get_user_by_login_or_email(
        &grafana_conf,
        user_email.as_str().to_lowercase().as_str(),
    )
    .await
    {
        Ok(user) => match user.id {
            Some(id) => id,
            None => {
                warn!("Failed to get id for user: {:?}", user);
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    CloudApiErrors::UserDoesNotExistInGrafana.to_string(),
                ));
            }
        },
        Err(_) => {
            warn!("Failed to get user: {:?}", user_email);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::UserDoesNotExistInGrafana.to_string(),
            ));
        }
    };

    for team_id in non_owned_team_grafana_ids {
        match get_team_by_id(&grafana_conf, team_id).await {
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

        match remove_team_member(&grafana_conf, team_id, user_id).await {
            Ok(_) => (),
            Err(err) => {
                warn!(
                    "Failed to remove user from team: {:?}, team_id: {:?}",
                    err, team_id
                );
                return Err(handle_grafana_error(err));
            }
        }
    }

    match admin_delete_user(&grafana_conf, user_id).await {
        Ok(_) => (),
        Err(err) => {
            warn!("Failed to delete user: {:?}", err);
            return Err(handle_grafana_error(err));
        }
    }

    Ok(())
}
