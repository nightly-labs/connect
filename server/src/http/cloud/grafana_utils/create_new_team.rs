use crate::structs::cloud::{
    api_cloud_errors::CloudApiErrors, grafana_error::handle_grafana_error,
};
use axum::http::StatusCode;
use log::warn;
use openapi::{
    apis::{
        configuration::Configuration, folder_permissions_api::update_folder_permissions,
        folders_api::create_folder, teams_api::create_team,
    },
    models::{
        CreateFolderCommand, CreateTeamCommand, DashboardAclUpdateItem, UpdateDashboardAclCommand,
    },
};
use std::sync::Arc;

pub async fn handle_grafana_create_new_team(
    grafana_conf: &Arc<Configuration>,
    admin_email: &String,
    team_name: &String,
) -> Result<i64, (StatusCode, String)> {
    let grafana_team_name = format!("[{}][{}]", team_name, admin_email);

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::env::{GRAFANA_API_KEY, GRAFANA_BASE_PATH};
    use openapi::apis::configuration::{ApiKey, Configuration};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_handle_grafana_create_new_team() {
        let team_name = "test_team_name".to_string();
        let email = "test507@gmail.com".to_string();

        let mut conf = Configuration::new();
        conf.base_path = GRAFANA_BASE_PATH().to_string();
        // conf.api_key = Some(ApiKey {
        //     prefix: Some("Bearer".to_string()),
        //     key: GRAFANA_API_KEY().to_string(),
        // });
        conf.basic_auth = Some(("admin".to_string(), Some("admin".to_string())));

        let grafana_client_conf = Arc::new(conf);

        let team_id = handle_grafana_create_new_team(&grafana_client_conf, &email, &team_name)
            .await
            .unwrap();

        println!("Team ID: {}", team_id);
    }
}
