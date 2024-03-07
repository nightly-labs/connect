use crate::{
    auth::AuthToken,
    env::{JWT_SECRET, NONCE},
    structs::api_cloud_errors::CloudApiErrors,
};
use axum::{
    extract::{ConnectInfo, State},
    http::StatusCode,
    Json,
};
use database::db::Db;
use garde::Validate;
use log::error;
use pwhash::bcrypt;
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS, Validate)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpLoginRequest {
    #[garde(email)]
    pub email: String,
    #[garde(ascii, length(min = 6, max = 30))]
    pub password: String,
    #[garde(skip)]
    pub enforce_ip: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct HttpLoginResponse {
    pub user_id: String,
    pub auth_token: String,
    pub refresh_token: String,
}

pub async fn login_with_password(
    ConnectInfo(ip): ConnectInfo<SocketAddr>,
    State(db): State<Option<Arc<Db>>>,
    Json(request): Json<HttpLoginRequest>,
) -> Result<Json<HttpLoginResponse>, (StatusCode, String)> {
    // Db connection has already been checked in the middleware
    let db = db.as_ref().ok_or((
        StatusCode::INTERNAL_SERVER_ERROR,
        CloudApiErrors::CloudFeatureDisabled.to_string(),
    ))?;

    // Check if user exists
    let user = match db.get_user_by_email(&request.email).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Err((
                StatusCode::BAD_REQUEST,
                CloudApiErrors::UserDoesNotExist.to_string(),
            ));
        }
        Err(err) => {
            error!("Failed to get user by email: {:?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::DatabaseError.to_string(),
            ));
        }
    };

    // Verify password
    if bcrypt::verify(
        format!("{}_{}", NONCE(), request.password),
        &user.password_hash,
    ) == false
    {
        return Err((
            StatusCode::BAD_REQUEST,
            CloudApiErrors::IncorrectPassword.to_string(),
        ));
    }

    // Generate tokens
    let ip = if request.enforce_ip { Some(ip) } else { None };
    // Access token
    let token = match AuthToken::new_access(&user.user_id, ip).encode(JWT_SECRET()) {
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
    let refresh_token = match AuthToken::new_refresh(&user.user_id, ip).encode(JWT_SECRET()) {
        Ok(token) => token,
        Err(err) => {
            error!("Failed to create refresh token: {:?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::RefreshTokenFailure.to_string(),
            ));
        }
    };

    return Ok(Json(HttpLoginResponse {
        auth_token: token,
        refresh_token,
        user_id: user.user_id,
    }));
}
