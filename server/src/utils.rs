use axum::http::{header, Method};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tower_http::cors::{Any, CorsLayer};

use crate::structs::{wallet_metadata::WalletMetadata, wallets::*};

pub fn get_timestamp_in_milliseconds() -> u64 {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
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
