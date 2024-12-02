use crate::{
    http::cloud::grafana_utils::{
        import_template_dashboard::setup_templates_dashboard,
        setup_database_datasource::setup_database_datasource,
        setup_template_folder::setup_templates_folder,
    },
    structs::{wallet_metadata::WalletMetadata, wallets::*},
};
use axum::http::{header, Method};
use log::error;
use openapi::apis::configuration::Configuration;
use std::{
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tower_http::cors::{Any, CorsLayer};

pub fn get_timestamp_in_milliseconds() -> u64 {
    let now = SystemTime::now();
    let since_the_epoch = match now.duration_since(UNIX_EPOCH) {
        Ok(duration) => duration,
        Err(err) => {
            error!(
                "Error getting timestamp in milliseconds: {}. Time went backwards",
                err
            );
            return 0;
        }
    };
    since_the_epoch.as_millis() as u64
}
pub fn get_cors() -> CorsLayer {
    CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_origin(Any)
        .allow_headers(vec![
            header::AUTHORIZATION,
            header::ACCEPT,
            header::CONTENT_TYPE,
        ])
        .max_age(Duration::from_secs(86400))
        .vary(vec![
            header::ORIGIN,
            header::ACCESS_CONTROL_REQUEST_METHOD,
            header::ACCESS_CONTROL_REQUEST_HEADERS,
        ])
}

pub fn get_wallets_metadata_vec() -> Vec<WalletMetadata> {
    vec![
        nightly_metadata(),
        polkadot_js_metadata(),
        talisman_metadata(),
        aleph_zero_signer_metadata(),
        subwallet_metadata(),
    ]
}

// CHECK THIS - used only at the begginning - better to have error
pub async fn import_template_dashboards(grafana_client: &Arc<Configuration>) {
    // Check if folder exists if not create it
    setup_templates_folder(&grafana_client).await.unwrap();

    // Check if database datasource was set up in grafana
    setup_database_datasource(&grafana_client).await.unwrap();

    // Check if template dashboard exists if not create it
    setup_templates_dashboard(&grafana_client).await.unwrap();

    // Setup global dashboard
    // TODO
}
