use crate::{
    env::DATABASE_ADDRESS,
    infra_env::{DATABASE_PORT, GRAFANA_DB_PASSWORD, GRAFANA_DB_USERNAME, POSTGRES_DB},
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
        let mut json_data = serde_json::Map::new();
        json_data.insert("sslmode".to_string(), "disable".into());
        json_data.insert("postgresVersion".to_string(), 1500.into());
        json_data.insert("timescaledb".to_string(), true.into());
        json_data.insert("maxOpenConns".to_string(), 100.into());
        json_data.insert("maxIdleConns".to_string(), 100.into());
        json_data.insert("maxIdleConnsAuto".to_string(), true.into());
        json_data.insert("connMaxLifetime".to_string(), 14400.into());
        json_data.insert("database".to_string(), POSTGRES_DB().into());

        let mut secure_settings = std::collections::HashMap::new();
        secure_settings.insert("password".to_string(), GRAFANA_DB_PASSWORD().to_string());

        let request_payload = AddDataSourceCommand {
            name: Some("Postgres".to_string()),
            r#type: Some("grafana-postgresql-datasource".to_string()),
            access: Some("proxy".to_string()),
            // DATABASE ADDRESS from main env file
            url: Some(format!(
                "{}:{}",
                DATABASE_ADDRESS().to_string(),
                DATABASE_PORT()
            )),
            database: Some(POSTGRES_DB().to_string()),
            user: Some(GRAFANA_DB_USERNAME().to_string()),
            basic_auth: None,
            with_credentials: Some(false),
            is_default: Some(true),
            json_data: Some(serde_json::Value::Object(json_data)),
            uid: Some(POSTGRES_DATASOURCE_UID.to_string()),
            basic_auth_user: None,
            secure_json_data: Some(secure_settings),
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
