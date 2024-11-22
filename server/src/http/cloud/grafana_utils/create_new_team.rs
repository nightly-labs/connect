use crate::structs::cloud::{
    api_cloud_errors::CloudApiErrors, grafana_error::handle_grafana_error,
};
use axum::http::StatusCode;
use log::warn;
use openapi::{
    apis::{
        admin_users_api::admin_create_user, configuration::Configuration,
        folder_permissions_api::update_folder_permissions, folders_api::create_folder,
        teams_api::create_team, users_api::get_user_by_login_or_email,
    },
    models::{
        AdminCreateUserForm, CreateFolderCommand, CreateTeamCommand, DashboardAclUpdateItem,
        UpdateDashboardAclCommand,
    },
};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::sync::Arc;

pub async fn handle_grafana_create_new_team(
    grafana_conf: &Arc<Configuration>,
    admin_email: &String,
    team_name: &String,
) -> Result<i64, (StatusCode, String)> {
    let grafana_team_name = format!("[{}][{}]", team_name, admin_email);

    // Check if user exists, if not create a new user
    if let Err(_) = get_user_by_login_or_email(&grafana_conf, admin_email).await {
        // Create user with the same email as the user, password can be anything, it won't be used
        let random_password: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();

        let request = AdminCreateUserForm {
            password: Some(random_password),
            email: Some(admin_email.to_lowercase().clone()),
            login: None,
            name: None,
            org_id: None,
        };

        match admin_create_user(&grafana_conf, request).await {
            Ok(_) => (),
            Err(err) => {
                warn!("Failed to create user: {:?}", err);
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    CloudApiErrors::InternalServerError.to_string(),
                ));
            }
        }
    }

    // create new team
    let team_request = CreateTeamCommand {
        email: Some(admin_email.clone()),
        name: Some(grafana_team_name.clone()),
    };

    let grafana_team_id = match create_team(&grafana_conf, team_request).await {
        Ok(response) => match response.team_id {
            Some(team_id) => team_id,
            None => {
                warn!("Failed to create team: {:?}", response);
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    CloudApiErrors::FailedToCreateTeam.to_string(),
                ));
            }
        },
        Err(err) => {
            return Err(handle_grafana_error(err));
        }
    };

    // create folder for team dashboards
    let folder_request = CreateFolderCommand {
        description: None,
        parent_uid: None,
        title: Some(grafana_team_name.clone()),
        uid: Some(grafana_team_id.to_string()),
    };

    let folder_uid = match create_folder(&grafana_conf, folder_request).await {
        Ok(response) => match response.uid {
            Some(folder_uid) => folder_uid,
            None => {
                warn!("Failed to create folder: {:?}", response);
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    CloudApiErrors::FailedToCreateTeam.to_string(),
                ));
            }
        },
        Err(err) => {
            return Err(handle_grafana_error(err));
        }
    };

    // set folder permissions for the whole team
    let update_permissions_request = UpdateDashboardAclCommand {
        items: Some(vec![DashboardAclUpdateItem {
            permission: Some(1), // Grant View permission for the whole team
            role: None,
            team_id: Some(grafana_team_id),
            user_id: None,
        }]),
    };

    if let Err(err) =
        update_folder_permissions(&grafana_conf, &folder_uid, update_permissions_request).await
    {
        return Err(handle_grafana_error(err));
    }

    Ok(grafana_team_id)
}
