use crate::{
    statics::DASHBOARD_TEMPLATE_UID,
    structs::cloud::{api_cloud_errors::CloudApiErrors, grafana_error::handle_grafana_error},
};
use axum::http::StatusCode;
use log::warn;
use openapi::{
    apis::{
        configuration::Configuration,
        dashboards_api::{get_dashboard_by_uid, import_dashboard},
    },
    models::ImportDashboardRequest,
};
use serde_json::json;
use std::sync::Arc;

pub async fn handle_grafana_create_new_app(
    grafana_conf: &Arc<Configuration>,
    app_name: &String,
    app_id: &String,
    team_id: &String,
) -> Result<(), (StatusCode, String)> {
    // Import template dashboard
    let mut template_dashboard =
        match get_dashboard_by_uid(&grafana_conf, &DASHBOARD_TEMPLATE_UID).await {
            Ok(response) => match response.dashboard {
                Some(dashboard) => dashboard,
                None => {
                    warn!("Failed to get template dashboard: {:?}", response);
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        CloudApiErrors::DashboardImportFail.to_string(),
                    ));
                }
            },
            Err(err) => {
                return Err(handle_grafana_error(err));
            }
        };

    // Modify dashboard template fields
    if let Some(uid_field) = template_dashboard.get_mut("uid") {
        *uid_field = json!(app_id);
    } else {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            CloudApiErrors::DashboardImportFail.to_string(),
        ));
    }

    if let Some(id_field) = template_dashboard.get_mut("id") {
        *id_field = json!(""); // Set dashboard id to empty string to create a new dashboard
    } else {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            CloudApiErrors::DashboardImportFail.to_string(),
        ));
    }

    if let Some(title_field) = template_dashboard.get_mut("title") {
        *title_field = json!(app_name);
    } else {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            CloudApiErrors::DashboardImportFail.to_string(),
        ));
    }

    // Import dashboard for the team
    if let Err(err) = import_dashboard(
        &grafana_conf,
        ImportDashboardRequest {
            dashboard: Some(template_dashboard),
            folder_id: None,
            folder_uid: Some(team_id.clone()), // When we create a new team, we create a folder with the same uid as the team id
            inputs: None,
            overwrite: Some(false),
            path: None,
            plugin_id: None,
        },
    )
    .await
    {
        return Err(handle_grafana_error(err));
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::env::{GRAFANA_API_KEY, GRAFANA_BASE_PATH};
    use crate::http::cloud::grafana_utils::create_new_app::handle_grafana_create_new_app;
    use openapi::apis::configuration::{ApiKey, Configuration};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_handle_grafana_create_new_app() {
        let team_id = "38".to_string();
        let app_name = "test_app_name".to_string();
        let app_id = "test_app_id".to_string();

        let mut conf = Configuration::new();
        conf.base_path = GRAFANA_BASE_PATH().to_string();
        conf.api_key = Some(ApiKey {
            prefix: Some("Bearer".to_string()),
            key: GRAFANA_API_KEY().to_string(),
        });

        let grafana_client_conf = Arc::new(conf);

        handle_grafana_create_new_app(&grafana_client_conf, &app_name, &app_id, &team_id)
            .await
            .unwrap();

        println!("WTF");
    }
}
