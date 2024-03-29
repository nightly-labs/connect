use crate::{
    env::NONCE,
    structs::cloud::api_cloud_errors::CloudApiErrors,
    utils::{generate_tokens, validate_request},
};
use axum::{
    extract::{ConnectInfo, State},
    http::StatusCode,
    Json,
};
use database::{
    db::Db,
    tables::{grafana_users::table_struct::GrafanaUser, utils::get_current_datetime},
};
use garde::Validate;
use log::error;
use pwhash::bcrypt;
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use ts_rs::TS;
use uuid7::uuid7;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct LoginWithGoogleResponse {
    pub user_id: String,
    pub auth_token: String,
    pub refresh_token: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS, Validate)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpLoginWithGoogleRequest {
    #[garde(ascii, length(min = 6, max = 300))]
    oauth_token: String,
    #[garde(email)]
    email: String,
    #[garde(skip)]
    enforce_ip: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GoogleResponse {
    id: String,
    email: String,
    verified_email: bool,
}

pub async fn login_with_google(
    ConnectInfo(ip): ConnectInfo<SocketAddr>,
    State(db): State<Option<Arc<Db>>>,
    Json(request): Json<HttpLoginWithGoogleRequest>,
) -> Result<Json<LoginWithGoogleResponse>, (StatusCode, String)> {
    // Db connection has already been checked in the middleware
    let db = db.as_ref().ok_or((
        StatusCode::INTERNAL_SERVER_ERROR,
        CloudApiErrors::CloudFeatureDisabled.to_string(),
    ))?;

    // Validate request
    validate_request(&request, &())?;

    // Get data from google and validate the payload
    let google_user_data = get_google_data(&request.oauth_token).await?;

    // Check if email is the same
    if google_user_data.email != request.email {
        return Err((
            StatusCode::BAD_REQUEST,
            CloudApiErrors::AccessTokenFailure.to_string(),
        ));
    };

    // Check if user is already registered
    match db.get_user_by_email(&request.email).await {
        Ok(Some(user)) => {
            let (auth_token, refresh_token) =
                generate_tokens(request.enforce_ip, ip, &user.user_id)?;

            return Ok(Json(LoginWithGoogleResponse {
                user_id: user.user_id,
                auth_token: auth_token,
                refresh_token: refresh_token,
            }));
        }
        Ok(None) => {
            // Generate random password
            let random_password: String = thread_rng()
                .sample_iter(&Alphanumeric)
                .take(30)
                .map(char::from)
                .collect();
            let hashed_password = bcrypt::hash(format!("{}_{}", NONCE(), random_password))
                .map_err(|e| {
                    error!("Failed to hash password: {:?}", e);
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        CloudApiErrors::InternalServerError.to_string(),
                    )
                })?;

            // Register user
            let user_id = uuid7().to_string();
            let grafana_user = GrafanaUser {
                user_id: user_id.clone(),
                email: request.email.clone(),
                password_hash: hashed_password,
                creation_timestamp: get_current_datetime(),
            };
            if let Err(err) = db.add_new_user(&grafana_user).await {
                error!("Failed to create user: {:?}", err);
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    CloudApiErrors::DatabaseError.to_string(),
                ));
            }

            // Generate tokens
            let (auth_token, refresh_token) = generate_tokens(request.enforce_ip, ip, &user_id)?;

            return Ok(Json(LoginWithGoogleResponse {
                user_id: user_id,
                auth_token: auth_token,
                refresh_token: refresh_token,
            }));
        }
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::DatabaseError.to_string(),
            ))
        }
    }
}

async fn get_google_data(oauth_token: &str) -> Result<GoogleResponse, (StatusCode, String)> {
    let google_user_data = reqwest::get(&format!(
        "https://www.googleapis.com/oauth2/v1/userinfo?access_token={}",
        oauth_token
    ))
    .await
    .map_err(|error| {
        (
            StatusCode::BAD_REQUEST,
            format!(
                "{} {}",
                CloudApiErrors::AccessTokenFailure.to_string(),
                error
            ),
        )
    })?
    .json::<GoogleResponse>()
    .await
    .map_err(|error| {
        (
            StatusCode::BAD_REQUEST,
            format!(
                "{} {}",
                CloudApiErrors::AccessTokenFailure.to_string(),
                error
            ),
        )
    })?;
    Ok(google_user_data)
}
