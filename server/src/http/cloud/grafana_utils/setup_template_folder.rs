use crate::{statics::TEMPLATES_FOLDER_UID, structs::cloud::grafana_error::handle_grafana_error};
use axum::http::StatusCode;
use openapi::{
    apis::{
        configuration::Configuration,
        folders_api::{create_folder, get_folder_by_uid},
    },
    models::CreateFolderCommand,
};
use std::sync::Arc;

pub async fn setup_templates_folder(
    grafana_conf: &Arc<Configuration>,
) -> Result<(), (StatusCode, String)> {
    // Check if folder exists if not create it
    match get_folder_by_uid(&grafana_conf, &TEMPLATES_FOLDER_UID).await {
        Ok(folder) => match folder.uid {
            Some(_uid) => return Ok(()),
            None => {
                // Try to create the folder
                let folder_request = CreateFolderCommand {
                    description: None,
                    title: Some("TEMPLATES_FOLDER".to_string()),
                    parent_uid: None,
                    uid: Some(TEMPLATES_FOLDER_UID.to_string()),
                };

                match create_folder(grafana_conf, folder_request).await {
                    Ok(_) => return Ok(()),
                    Err(err) => {
                        println!("Failed to create folder: {:?}", err);
                        return Err(handle_grafana_error(err));
                    }
                }
            }
        },
        Err(err) => {
            println!("Failed to get templates folder: {:?}", err);

            // Try to create the folder anyway
            let folder_request = CreateFolderCommand {
                description: None,
                title: Some("TEMPLATES_FOLDER".to_string()),
                parent_uid: None,
                uid: Some(TEMPLATES_FOLDER_UID.to_string()),
            };

            match create_folder(grafana_conf, folder_request).await {
                Ok(_) => return Ok(()),
                Err(err) => {
                    println!("Failed to create folder: {:?}", err);
                    return Err(handle_grafana_error(err));
                }
            }
        }
    }
}
