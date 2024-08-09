use crate::{
    env::DATABASE_ADDRESS,
    infra_env::{POSTGRES_DB, POSTGRES_USER},
    statics::POSTGRES_DATASOURCE_UID,
    structs::cloud::grafana_error::handle_grafana_error,
};
use axum::http::StatusCode;
use openapi::{
    apis::{
        configuration::Configuration,
        datasources_api::{add_data_source, get_data_source_by_uid},
    },
    models::AddDataSourceCommand,
};
use std::sync::Arc;

pub async fn setup_database_datasource(
    grafana_conf: &Arc<Configuration>,
) -> Result<(), (StatusCode, String)> {
    // Check if datasource already exists,  otherwise create it
    if let Err(_) = get_data_source_by_uid(grafana_conf, POSTGRES_DATASOURCE_UID).await {
        let request_payload = AddDataSourceCommand {
            name: Some("Postgres".to_string()),
            r#type: Some("postgres".to_string()),
            access: Some("proxy".to_string()),
            // DATABASE ADDRESS from main env file
            url: Some(DATABASE_ADDRESS().to_string()),
            database: Some(POSTGRES_DB().to_string()),
            user: Some(POSTGRES_USER().to_string()),
            basic_auth: None,
            with_credentials: Some(false),
            is_default: Some(true),
            json_data: None,
            uid: Some(POSTGRES_DATASOURCE_UID.to_string()),
            basic_auth_user: None,
            secure_json_data: None,
        };

        match add_data_source(&grafana_conf, request_payload).await {
            Ok(_) => return Ok(()),
            Err(err) => {
                println!("Failed to import database datasource: {:?}", err);
                return Err(handle_grafana_error(err));
            }
        }
    }

    Ok(())
}
