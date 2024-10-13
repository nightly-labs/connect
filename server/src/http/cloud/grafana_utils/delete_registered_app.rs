use crate::structs::cloud::{
    api_cloud_errors::CloudApiErrors, grafana_error::handle_grafana_error,
};
use axum::http::StatusCode;
use log::warn;
use openapi::apis::{
    configuration::Configuration,
    dashboards_api::{delete_dashboard_by_uid, get_dashboard_by_uid},
};
use std::sync::Arc;

pub async fn handle_grafana_delete_app(
    grafana_conf: &Arc<Configuration>,
    app_id: &String,
) -> Result<(), (StatusCode, String)> {
    match get_dashboard_by_uid(&grafana_conf, &app_id).await {
        Ok(response) => match response.dashboard {
            Some(_) => (),
            None => {
                warn!(
                    "Failed to get dashboard: {:?}, dashboard_id:{:?}",
                    response, app_id
                );
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    CloudApiErrors::AppDoesNotExist.to_string(),
                ));
            }
        },
        Err(err) => {
            warn!(
                "Failed to delete dashboard: {:?}, dashboard_id: {:?}",
                err, app_id
            );
            return Err(handle_grafana_error(err));
        }
    };

    match delete_dashboard_by_uid(&grafana_conf, &app_id).await {
        Ok(_) => return Ok(()),
        Err(err) => {
            warn!(
                "Failed to delete dashboard: {:?}, dashboard_id: {:?}",
                err, app_id
            );
            return Err(handle_grafana_error(err));
        }
    }
}
