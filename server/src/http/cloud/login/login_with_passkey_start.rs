use crate::{
    http::cloud::utils::validate_request,
    structs::{
        cloud::api_cloud_errors::CloudApiErrors,
        session_cache::{
            ApiSessionsCache, PasskeyLoginVerification, SessionCache, SessionsCacheKey,
        },
    },
    utils::get_timestamp_in_milliseconds,
};
use axum::{extract::State, http::StatusCode, Json};
use database::db::Db;
use garde::Validate;
use log::error;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ts_rs::TS;
use webauthn_rs::{prelude::RequestChallengeResponse, Webauthn};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS, Validate)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpLoginWithPasskeyStartRequest {
    #[garde(email)]
    pub email: String,
}

pub type HttpLoginWithPasskeyStartResponse = RequestChallengeResponse;

pub async fn login_with_passkey_start(
    State(db): State<Arc<Db>>,
    State(web_auth): State<Arc<Webauthn>>,
    State(sessions_cache): State<Arc<ApiSessionsCache>>,
    Json(request): Json<HttpLoginWithPasskeyStartRequest>,
) -> Result<Json<HttpLoginWithPasskeyStartResponse>, (StatusCode, String)> {
    // Validate request
    validate_request(&request, &())?;

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

    // Check if user has passkey
    let passkeys = match user.passkeys {
        Some(passkeys) => passkeys,
        None => {
            return Err((
                StatusCode::BAD_REQUEST,
                CloudApiErrors::PasswordNotSet.to_string(),
            ));
        }
    };

    // Save to cache passkey challenge request
    let sessions_key = SessionsCacheKey::PasskeyLogin(request.email.clone()).to_string();

    // Remove leftover session data
    sessions_cache.remove(&sessions_key);

    match web_auth.start_passkey_authentication(&passkeys) {
        Ok((rcr, auth_state)) => {
            sessions_cache.set(
                sessions_key,
                SessionCache::PasskeyLogin(PasskeyLoginVerification {
                    email: request.email.clone(),
                    passkey_verification_state: auth_state,
                    created_at: get_timestamp_in_milliseconds(),
                }),
                None,
            );
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
