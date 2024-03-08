use crate::{
    statics::{NAME_REGEX, REGISTER_PASSWORD_VALIDATOR},
    structs::{wallet_metadata::WalletMetadata, wallets::*},
};
use axum::http::{header, Method, StatusCode};
use garde::Validate;
use std::{
    str::FromStr,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tower_http::cors::{Any, CorsLayer};
use uuid7::Uuid;

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

pub fn validate_request<T>(payload: T, ctx: &T::Context) -> Result<(), (StatusCode, String)>
where
    T: Validate,
{
    payload.validate(ctx).map_err(|report| {
        let error_message = match report.iter().next() {
            Some((_, error)) => error.message().to_string(),
            None => "Unknown error".to_string(),
        };

        (StatusCode::BAD_REQUEST, format!("{}", error_message))
    })?;
    return Ok(());
}

pub fn custom_validate_uuid(string_uuid: &String, _context: &()) -> garde::Result {
    Uuid::from_str(&string_uuid)
        .map_err(|_| garde::Error::new("Invalid UUID format".to_string()))?;
    Ok(())
}

pub fn custom_validate_name(name: &String, _context: &()) -> garde::Result {
    NAME_REGEX
        .is_match(name)
        .then(|| ())
        .ok_or_else(|| garde::Error::new("App name must be 3-30 characters long and include only alphanumeric characters, underscores, or slashes.".to_string()))
}

pub fn custom_validate_new_password(password: &String, _context: &()) -> garde::Result {
    if !password.is_ascii() {
        return Err(garde::Error::new("Password contains non-ascii characters"));
    }
    for validator in REGISTER_PASSWORD_VALIDATOR.iter() {
        if !validator.re.is_match(password) {
            return Err(garde::Error::new(validator.error.as_str()));
        }
    }
    Ok(())
}
