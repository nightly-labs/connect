use crate::{
    auth::AuthToken,
    env::{is_env_production, JWT_SECRET, NONCE},
    ip_geolocation::GeolocationRequester,
    statics::{CODE_REGEX, NAME_REGEX, REGISTER_PASSWORD_VALIDATOR},
    structs::cloud::api_cloud_errors::CloudApiErrors,
    test_env::is_test_env,
    utils::get_timestamp_in_milliseconds,
};
use addr::parse_domain_name;
use anyhow::bail;
use axum::http::StatusCode;
use database::{
    db::Db,
    structs::{
        consts::DAY_IN_SECONDS, geo_location::Geolocation, pagination_cursor::PaginationCursor,
    },
    tables::{ip_addresses::table_struct::IpAddressEntry, utils::get_current_datetime},
};
use garde::Validate;
use log::{error, info, warn};
use rand::{
    distributions::{Alphanumeric, Uniform},
    Rng,
};
use reqwest::Url;
use sha256::digest;
use std::{net::SocketAddr, str::FromStr, sync::Arc, time::Duration};
use uuid7::Uuid;

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

pub fn custom_validate_team_id(string_id: &String, _context: &()) -> garde::Result {
    // For now we are using i64 returned from grafana as team_id, hopefully this will be changed to UUID
    match i64::from_str(string_id) {
        Ok(id) => {
            if id < 0 {
                return Err(garde::Error::new("Invalid ID format".to_string()));
            }
        }
        Err(_) => return Err(garde::Error::new("Invalid ID format".to_string())),
    }
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

pub fn custom_validate_optional_pagination_cursor(
    cursor: &Option<PaginationCursor>,
    _context: &(),
) -> garde::Result {
    match cursor {
        Some(cursor) => {
            if cursor.0.is_empty() {
                return Err(garde::Error::new(
                    CloudApiErrors::InvalidPaginationCursor.to_string(),
                ));
            }
            Ok(())
        }
        None => Ok(()),
    }
}

pub fn custom_validate_verification_code(name: &String, _context: &()) -> garde::Result {
    CODE_REGEX.is_match(name).then(|| ()).ok_or_else(|| {
        garde::Error::new(CloudApiErrors::InvalidOrExpiredVerificationCode.to_string())
    })
}

pub fn generate_verification_code() -> String {
    if !is_env_production() || is_test_env() {
        return "123456".to_string();
    }

    let mut rng = rand::thread_rng();
    let range = Uniform::new(0, 10);
    let code = (0..6).map(|_| rng.sample(&range).to_string()).collect();
    code
}

pub fn generate_authentication_code() -> (String, String) {
    let auth_code: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    let encrypted_auth_code = digest(format!("{}{}", NONCE(), auth_code));

    (auth_code, encrypted_auth_code)
}

pub fn check_verification_code(code: &String, verification_code: &String, created_at: u64) -> bool {
    if code != verification_code {
        return false;
    }

    let current_time = get_timestamp_in_milliseconds();
    let time_diff = current_time - created_at;

    // Code expires after 5 minutes (5 * 60 * 1000 = 300_000 ms)
    if time_diff > 300_000 {
        return false;
    }

    true
}

pub fn check_auth_code(
    auth_code: &String,
    encrypted_auth_code: &Option<String>,
    created_at: u64,
) -> bool {
    if is_test_env() {
        if encrypted_auth_code.is_none() {
            println!("Encrypted auth code is missing");
        }

        return true;
    }

    let encrypted_auth_code = match encrypted_auth_code {
        Some(auth_code) => auth_code,
        None => return false,
    };

    if encrypted_auth_code != &digest(format!("{}{}", NONCE(), auth_code)) {
        return false;
    }

    let current_time = get_timestamp_in_milliseconds();
    let time_diff = current_time - created_at;

    // Code expires after 5 minutes (5 * 60 * 1000 = 300_000 ms)
    if time_diff > 300_000 {
        return false;
    }

    true
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

    // Skip requesting geolocation data if the ip is localhost
    if ip.ip().is_loopback() {
        info!("Skipping geolocation request for localhost ip: [{}]", ip);
        return None;
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

pub fn refresh_auth_token(
    refresh_token: AuthToken,
    ip: Option<SocketAddr>,
) -> Result<String, (StatusCode, String)> {
    match AuthToken::new_access(&refresh_token.user_id, ip).encode(JWT_SECRET()) {
        Ok(token) => return Ok(token),
        Err(err) => {
            error!("Failed to create access token: {:?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::AccessTokenFailure.to_string(),
            ));
        }
    };
}

pub fn custom_validate_domain_name(domain_name: &String) -> anyhow::Result<String> {
    // Check if the domain name is empty
    if domain_name.trim().is_empty() {
        warn!("Domain name is empty: {:?}", domain_name);
        bail!(CloudApiErrors::InvalidDomainName);
    }

    match parse_domain_name(domain_name) {
        Ok(name) => Ok(name.to_string()),
        Err(err) => {
            warn!("Failed to convert domain name to ascii: {:?}", err);
            bail!(CloudApiErrors::InvalidDomainName);
        }
    }
}

pub fn extract_domain_name(origin: &String) -> Result<String, String> {
    let parsed_url = Url::parse(origin).map_err(|err| {
        format!("Failed to parse origin: {:?}, err: {:?}", origin, err).to_string()
    })?;

    // Extract the domain name
    let domain_name = parsed_url.domain().ok_or_else(|| {
        format!(
            "Failed to extract domain from parsed_url: {:?}, origin: {:?}",
            parsed_url, origin
        )
        .to_string()
    })?;

    Ok(domain_name.to_string())
}

#[cfg(feature = "cloud_integration_tests")]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_ascii_domain() {
        let domain_name = String::from("example.com");
        assert!(custom_validate_domain_name(&domain_name).is_ok());
    }

    #[test]
    fn test_valid_unicode_domain() {
        let domain_name = String::from("münchen.de");
        assert!(custom_validate_domain_name(&domain_name).is_ok());
    }

    #[test]
    fn test_invalid_domain() {
        let domain_name = String::from("this is not a domain");
        assert!(custom_validate_domain_name(&domain_name).is_err());
    }

    #[test]
    fn test_empty_domain() {
        let domain_name = String::from("");
        assert!(custom_validate_domain_name(&domain_name).is_err());
    }
}
