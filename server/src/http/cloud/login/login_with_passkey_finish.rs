use crate::{
    http::cloud::utils::{generate_tokens, validate_request},
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
use log::{error, warn};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use ts_rs::TS;
use webauthn_rs::{prelude::PublicKeyCredential, Webauthn};

#[derive(Validate, Debug, Deserialize, Serialize)]
pub struct HttpLoginWithPasskeyFinishRequest {
    #[garde(email)]
    pub email: String,
    #[garde(skip)]
    pub credential: PublicKeyCredential,
    #[garde(skip)]
    pub enforce_ip: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpLoginWithPasskeyFinishResponse {
    pub user_id: String,
    pub auth_token: String,
    pub refresh_token: String,
}

pub async fn login_with_passkey_finish(
    ConnectInfo(ip): ConnectInfo<SocketAddr>,
    State(db): State<Arc<Db>>,
    State(web_auth): State<Arc<Webauthn>>,
    State(sessions_cache): State<Arc<ApiSessionsCache>>,
    Json(request): Json<HttpLoginWithPasskeyFinishRequest>,
) -> Result<Json<HttpLoginWithPasskeyFinishResponse>, (StatusCode, String)> {
    // Validate request
    validate_request(&request, &())?;

    // Get session data
    let sessions_key = SessionsCacheKey::PasskeyLogin(request.email.clone()).to_string();
    let session_data = match sessions_cache.get(&sessions_key) {
        Some(SessionCache::PasskeyLogin(session)) => session,
        _ => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::InternalServerError.to_string(),
            ));
        }
    };

    // Remove leftover session data
    sessions_cache.remove(&sessions_key);

    // Finish passkey authentication
    if let Err(err) = web_auth.finish_passkey_authentication(
        &request.credential,
        &session_data.passkey_verification_state,
    ) {
        warn!(
            "Failed to finish passkey authentication: {:?}, user_email: {}",
            err, request.email
        );
        return Err((
            StatusCode::BAD_REQUEST,
            CloudApiErrors::InvalidPasskeyCredential.to_string(),
        ));
    }

    // Get user data
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

    // Generate tokens
    let (auth_token, refresh_token) = generate_tokens(request.enforce_ip, ip, &user.user_id)?;

    return Ok(Json(HttpLoginWithPasskeyFinishResponse {
        auth_token,
        refresh_token,
        user_id: user.user_id,
    }));
}
