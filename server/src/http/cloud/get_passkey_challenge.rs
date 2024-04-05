use crate::{
    middlewares::auth_middleware::UserId,
    structs::{
        cloud::api_cloud_errors::CloudApiErrors,
        session_cache::{ApiSessionsCache, SessionCache, SessionsCacheKey},
    },
};
use axum::{extract::State, http::StatusCode};
use axum::{Extension, Json};
use database::db::Db;
use log::error;
use std::sync::Arc;
use webauthn_rs::prelude::RequestChallengeResponse;
use webauthn_rs::Webauthn;

pub type HttpTwoFactorWithPasskeyStartResponse = RequestChallengeResponse;

pub async fn get_passkey_challenge(
    State(db): State<Arc<Db>>,
    State(web_auth): State<Arc<Webauthn>>,
    State(sessions_cache): State<Arc<ApiSessionsCache>>,
    Extension(user_id): Extension<UserId>,
) -> Result<Json<HttpTwoFactorWithPasskeyStartResponse>, (StatusCode, String)> {
    // Get user data
    let user_data = match db.get_user_by_user_id(&user_id).await {
        Ok(Some(user_data)) => user_data,
        Ok(None) => {
            return Err((
                StatusCode::BAD_REQUEST,
                CloudApiErrors::UserDoesNotExist.to_string(),
            ))
        }
        Err(err) => {
            error!(
                "Failed to check if user exists: {:?}, user_id: {}",
                err, user_id
            );
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::DatabaseError.to_string(),
            ));
        }
    };

    let passkey = match user_data.passkeys {
        Some(passkey) => passkey,
        None => {
            return Err((
                StatusCode::BAD_REQUEST,
                CloudApiErrors::UserDoesNotHavePasskey.to_string(),
            ));
        }
    };

    // Save to cache passkey challenge request
    let sessions_key = SessionsCacheKey::PasskeyVerification(user_id.clone()).to_string();

    // Remove leftover session data
    sessions_cache.remove(&sessions_key);

    match web_auth.start_passkey_authentication(&passkey) {
        Ok((rcr, auth_state)) => {
            sessions_cache.set(sessions_key, SessionCache::Passkey2FA(auth_state), None);
            return Ok(Json(rcr));
        }
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::WebAuthnError.to_string(),
            ));
        }
    };
}
