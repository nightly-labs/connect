use crate::{
    statics::{DASHBOARD_TEMPLATE_UID, TEMPLATES_FOLDER_UID},
    structs::cloud::grafana_error::handle_grafana_error,
};
use axum::http::StatusCode;
use openapi::{
    apis::{
        configuration::Configuration,
        dashboards_api::{get_dashboard_by_uid, import_dashboard},
    },
    models::ImportDashboardRequest,
};
use serde_json::Value;
use std::{env, sync::Arc};
use tokio::fs;

pub async fn setup_templates_dashboard(
    grafana_conf: &Arc<Configuration>,
) -> Result<(), (StatusCode, String)> {
    // Load file with dashboard
    let current_dir = env::current_dir().unwrap();
    // Construct the path to the JSON file
    let json_path = current_dir
        .join("..")
        .join("grafana")
        .join("TEMPLATE_DASHBOARD.json");

    let dashboard_blob = fs::read(json_path).await.expect("Failed to read file");
    let dashboard: Value =
        serde_json::from_slice(&dashboard_blob).expect("Failed to deserialize json");

    // Check if dashboard exists if not create it
    match get_dashboard_by_uid(&grafana_conf, &DASHBOARD_TEMPLATE_UID).await {
        Ok(response) => match response.dashboard {
            Some(_dashboard) => return Ok(()),
            None => {
                // Try to import the dashboard
                let request = ImportDashboardRequest {
                    dashboard: Some(dashboard),
                    folder_id: None,
                    folder_uid: Some(TEMPLATES_FOLDER_UID.to_string()),
                    overwrite: Some(false),
                    inputs: None,
                    path: None,
                    plugin_id: None,
                };

                match import_dashboard(&grafana_conf, request).await {
                    Ok(_) => return Ok(()),
                    Err(err) => {
                        println!("Failed to import template dashboard: {:?}", err);
                        return Err(handle_grafana_error(err));
                    }
                }
            }
        },
        Err(err) => {
            println!("Failed to import template dashboard: {:?}", err);

            // Try to import the dashboard anyway
            let request = ImportDashboardRequest {
                dashboard: Some(dashboard),
                folder_id: None,
                folder_uid: Some(TEMPLATES_FOLDER_UID.to_string()),
                overwrite: Some(false),
                inputs: None,
                path: None,
                plugin_id: None,
            };

            match import_dashboard(&grafana_conf, request).await {
                Ok(_) => return Ok(()),
                Err(err) => {
                    println!("Failed to import template dashboard: {:?}", err);
                    return Err(handle_grafana_error(err));
                }
            }
        }
    }
}
