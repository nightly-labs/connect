use crate::{
    auth::AuthToken,
    env::JWT_SECRET,
    ip_geolocation::GeolocationRequester,
    statics::{NAME_REGEX, REGISTER_PASSWORD_VALIDATOR},
    structs::{
        cloud::api_cloud_errors::CloudApiErrors, wallet_metadata::WalletMetadata, wallets::*,
    },
};
use axum::http::{header, Method, StatusCode};
use database::{
    db::Db, structs::consts::DAY_IN_SECONDS, tables::ip_addresses::table_struct::IpAddressEntry,
};
use database::{structs::geo_location::Geolocation, tables::utils::get_current_datetime};
use garde::Validate;
use log::{error, warn};
use std::{
    net::SocketAddr,
    str::FromStr,
    sync::Arc,
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

pub async fn get_geolocation_data(
    db: &Arc<Db>,
    geo_loc_requester: &Arc<GeolocationRequester>,
    ip: &SocketAddr,
) -> Option<Geolocation> {
    // Check if we already have the data in the database
    match db.get_ip_address(&ip.to_string()).await {
        Ok(Some(ip_address)) => {
            // Check if the data is not older than 24 hours
            let current_time = get_current_datetime();

            if ip_address.last_updated_at + Duration::from_secs(DAY_IN_SECONDS) > current_time {
                return Some(Geolocation {
                    country: ip_address.country,
                    city: ip_address.city,
                    lat: ip_address.lat,
                    lon: ip_address.lon,
                });
            }
        }
        Ok(None) => {
            // Do nothing, we will fetch the data from the geolocation service
        }
        Err(err) => {
            warn!("Failed to get geolocation, ip: [{}], err: [{}]", ip, err);
            return None;
        }
    }

    // Fetch data from the geolocation service and update the database
    match geo_loc_requester.get_geolocation(&ip.to_string()).await {
        Ok(geo_location) => match (geo_location.lat, geo_location.lon) {
            (Some(_), Some(_)) => {
                let ip_address_entry = IpAddressEntry {
                    ip_addr: ip.to_string(),
                    last_updated_at: get_current_datetime(),
                    country: geo_location.country.clone(),
                    city: geo_location.city.clone(),
                    lat: geo_location.lat.clone(),
                    lon: geo_location.lon.clone(),
                };

                // Try to safely insert the new ip address
                if let Err(err) = db.upsert_ip_address(&ip_address_entry).await {
                    warn!(
                        "Failed to insert new ip address, ip: [{}], err: [{}]",
                        ip, err
                    );
                }

                // Return the geolocation data, no matter if the we managed to save the data to the database
                Some(geo_location.into())
            }
            _ => {
                warn!(
                    "Failed to get geolocation, ip: [{}], err: [{}]",
                    ip, "Latitude or longitude is missing"
                );
                None
            }
        },
        Err(err) => {
            warn!("Failed to get geolocation, ip: [{}], err: [{}]", ip, err);
            None
        }
    }
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
        .ok_or_else(|| garde::Error::new(CloudApiErrors::InvalidName.to_string()))
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

pub fn generate_tokens(
    enforce_ip: bool,
    ip: SocketAddr,
    user_id: &String,
    // (Auth Token, Refresh Token)
) -> Result<(String, String), (StatusCode, String)> {
    // Generate tokens
    let ip = if enforce_ip { Some(ip) } else { None };
    // Access token
    let token = match AuthToken::new_access(&user_id, ip).encode(JWT_SECRET()) {
        Ok(token) => token,
        Err(err) => {
            error!("Failed to create access token: {:?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::AccessTokenFailure.to_string(),
            ));
        }
    };
    // Refresh token
    let refresh_token = match AuthToken::new_refresh(&user_id, ip).encode(JWT_SECRET()) {
        Ok(token) => token,
        Err(err) => {
            error!("Failed to create refresh token: {:?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::RefreshTokenFailure.to_string(),
            ));
        }
    };

    Ok((token, refresh_token))
}
