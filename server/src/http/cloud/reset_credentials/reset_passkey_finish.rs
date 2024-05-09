use crate::{
    env::is_env_production,
    http::cloud::utils::{custom_validate_verification_code, validate_request},
    structs::{
        cloud::api_cloud_errors::CloudApiErrors,
        session_cache::{ApiSessionsCache, SessionCache, SessionsCacheKey},
    },
};
use axum::{extract::State, http::StatusCode, Json};
use database::db::Db;
use garde::Validate;
use log::{error, warn};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ts_rs::TS;
use webauthn_rs::prelude::RegisterPublicKeyCredential;
use webauthn_rs::Webauthn;

#[derive(Validate, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpResetPasskeyFinishRequest {
    #[garde(email)]
    pub email: String,
    #[garde(skip)]
    pub credential: RegisterPublicKeyCredential,
    #[garde(custom(custom_validate_verification_code))]
    pub code: String,
}

#[derive(Validate, Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct HttpResetPasskeyFinishResponse {}

pub async fn reset_passkey_finish(
    State(db): State<Arc<Db>>,
    State(web_auth): State<Arc<Webauthn>>,
    State(sessions_cache): State<Arc<ApiSessionsCache>>,
    Json(request): Json<HttpResetPasskeyFinishRequest>,
) -> Result<Json<HttpResetPasskeyFinishResponse>, (StatusCode, String)> {
    // Validate request
    validate_request(&request, &())?;

    // Get cache data
    let sessions_key =
        SessionsCacheKey::ResetPasskeyVerification(request.email.clone()).to_string();
    let session_data = match sessions_cache.get(&sessions_key) {
        Some(SessionCache::ResetPasskey(session)) => session,
        _ => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::InternalServerError.to_string(),
            ));
        }
    };

    // Remove leftover session data
    sessions_cache.remove(&sessions_key);

    // validate code only on production
    if is_env_production() {
        // Validate the code
        if session_data.code != request.code {
            return Err((
                StatusCode::BAD_REQUEST,
                CloudApiErrors::InvalidVerificationCode.to_string(),
            ));
        }
    }

    // Validate passkey reset
    let passkey = match web_auth.finish_passkey_registration(
        &request.credential,
        &session_data.passkey_registration_state,
    ) {
        Ok(sk) => sk,
        Err(err) => {
            error!(
                "Failed to finish passkey reset: {:?}, user_email: {}",
                err, &request.email
            );
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::WebAuthnError.to_string(),
            ));
        }
    };

    // Validate new passkey
    // Get user data
    let user_data = match db.get_user_by_email(&request.email).await {
        Ok(Some(user_data)) => user_data,
        Ok(None) => {
            warn!("Reaching this place [Passkey reset finish, user does not exist] should not be possible as we have already checked it during reset start method, user email: {}", &request.email);
            return Err((
                StatusCode::BAD_REQUEST,
                CloudApiErrors::UserDoesNotExist.to_string(),
            ));
        }
        Err(err) => {
            error!(
                "Failed to get user data: {:?}, user_email: {}",
                err, &request.email
            );
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::DatabaseError.to_string(),
            ));
        }
    };

    // Check if user has already added this passkey
    let mut passkeys = match user_data.passkeys {
        Some(passkeys) => {
            if passkeys.contains(&passkey) {
                return Err((
                    StatusCode::BAD_REQUEST,
                    CloudApiErrors::PasskeyAlreadyExists.to_string(),
                ));
            }

            passkeys
        }
        None => {
            warn!("Reaching this place [Passkey reset finish, user does not have passkey] should not be possible as we have already checked it during reset start method, user email: {}", &request.email);
            return Err((
                StatusCode::BAD_REQUEST,
                CloudApiErrors::UserDoesNotHavePasskey.to_string(),
            ));
        }
    };

    // Add new passkey
    passkeys.push(passkey);

    // Update passkeys in database
    match db.update_passkeys(&request.email, &passkeys).await {
        Ok(_) => {
            return Ok(Json(HttpResetPasskeyFinishResponse {}));
        }
        Err(err) => {
            error!(
                "Failed to update user passkeys: {:?}, user_email: {}",
                err, &request.email
            );
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::DatabaseError.to_string(),
            ));
        }
    }
}
