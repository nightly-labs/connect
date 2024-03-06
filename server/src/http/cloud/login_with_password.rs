use crate::{auth::AuthToken, env::JWT_SECRET};
use axum::{
    extract::{ConnectInfo, State},
    http::StatusCode,
    Json,
};
use database::db::Db;
use pwhash::bcrypt;
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpLoginRequest {
    pub email: String,
    pub password: String,
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
        "Failed to get database connection".to_string(),
    ))?;

    // Check if user exists
    let user = db
        .get_user_by_email(&request.email)
        .await
        .map_err(|_| (StatusCode::BAD_REQUEST, "Could not find user".to_string()))?;

    // Verify password
    if bcrypt::verify(request.password, &user.password_hash) == false {
        return Err((StatusCode::BAD_REQUEST, "Incorrect Password".to_string()));
    }

    let ip = if request.enforce_ip { Some(ip) } else { None };
    let token = AuthToken::new_access(&user.user_id, ip)
        .encode(JWT_SECRET())
        .expect("Could not encode token");
    let refresh_token = AuthToken::new_refresh(&user.user_id, ip)
        .encode(JWT_SECRET())
        .expect("Could not encode token");

    return Ok(Json(HttpLoginResponse {
        auth_token: token,
        refresh_token,
        user_id: user.user_id,
    }));
}
