use crate::{
    http::cloud::utils::{check_auth_code, generate_tokens, validate_request},
    structs::{
        cloud::api_cloud_errors::CloudApiErrors,
        session_cache::{ApiSessionsCache, SessionCache, SessionsCacheKey},
    },
};
use axum::{
    extract::{ConnectInfo, State},
    http::StatusCode,
    Json,
};
use database::db::Db;
use garde::Validate;
use log::error;
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};

use ts_rs::TS;
use uuid7::uuid7;
use webauthn_rs::prelude::RegisterPublicKeyCredential;
use webauthn_rs::Webauthn;

#[derive(Validate, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpRegisterWithPasskeyFinishRequest {
    #[garde(email)]
    pub email: String,
    #[garde(skip)]
    pub credential: RegisterPublicKeyCredential,
    #[garde(skip)]
    pub auth_code: String,
    #[garde(skip)]
    pub enforce_ip: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpRegisterWithPasskeyFinishResponse {
    pub user_id: String,
    pub auth_token: String,
    pub refresh_token: String,
}

pub async fn register_with_passkey_finish(
    ConnectInfo(ip): ConnectInfo<SocketAddr>,
    State(db): State<Arc<Db>>,
    State(web_auth): State<Arc<Webauthn>>,
    State(sessions_cache): State<Arc<ApiSessionsCache>>,
    Json(request): Json<HttpRegisterWithPasskeyFinishRequest>,
) -> Result<Json<HttpRegisterWithPasskeyFinishResponse>, (StatusCode, String)> {
    // Validate request
    validate_request(&request, &())?;

    // Get cache data
    let sessions_key = SessionsCacheKey::PasskeyVerification(request.email.clone()).to_string();
    let session_data = match sessions_cache.get(&sessions_key) {
        Some(SessionCache::VerifyPasskeyRegister(session)) => session,
        _ => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::InternalServerError.to_string(),
            ));
        }
    };

    // Validate auth code
    if !check_auth_code(
        &request.auth_code,
        &session_data.authentication_code,
        session_data.created_at,
    ) {
        return Err((
            StatusCode::BAD_REQUEST,
            CloudApiErrors::InvalidOrExpiredAuthCode.to_string(),
        ));
    }

    // Validate passkey register
    let passkey = match web_auth.finish_passkey_registration(
        &request.credential,
        &session_data.passkey_registration_state,
    ) {
        Ok(sk) => sk,
        Err(err) => {
            error!("Failed to finish passkey registration: {:?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::WebAuthnError.to_string(),
            ));
        }
    };

    // Remove leftover session data
    sessions_cache.remove(&sessions_key);

    // Save user to database
    let user_id = uuid7().to_string();

    match db
        .add_new_user(
            &user_id,
            &request.email,
            // None for password
            None,
            Some(&passkey),
        )
        .await
    {
        Ok(_) => {
            // Generate tokens
            let (auth_token, refresh_token) =
                generate_tokens(request.enforce_ip, ip, &user_id, &request.email)?;
            return Ok(Json(HttpRegisterWithPasskeyFinishResponse {
                auth_token,
                refresh_token,
                user_id,
            }));
        }
        Err(err) => {
            error!("Failed to create user: {:?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::DatabaseError.to_string(),
            ));
        }
    }
}
