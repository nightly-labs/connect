use crate::{
    http::cloud::utils::{custom_validate_verification_code, validate_request},
    structs::{
        cloud::api_cloud_errors::CloudApiErrors,
        session_cache::{ApiSessionsCache, SessionCache, VerificationAction},
    },
};
use axum::{extract::State, http::StatusCode, Json};
use database::db::Db;
use garde::Validate;
use log::error;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ts_rs::TS;

use super::utils::{check_verification_code, generate_authentication_code};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS, Validate)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpVerifyCodeRequest {
    #[garde(email)]
    pub email: String,
    #[garde(custom(custom_validate_verification_code))]
    pub code: String,
    #[garde(skip)]
    pub action: VerificationAction,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpVerifyCodeResponse {
    pub verification_code: String,
}

pub async fn verify_code(
    State(db): State<Arc<Db>>,
    State(sessions_cache): State<Arc<ApiSessionsCache>>,
    Json(request): Json<HttpVerifyCodeRequest>,
) -> Result<Json<HttpVerifyCodeResponse>, (StatusCode, String)> {
    // Validate request
    validate_request(&request, &())?;

    // Check if user exists
    match db.get_user_by_email(&request.email).await {
        Ok(Some(_)) => {
            // Continue
        }
        Ok(None) => {
            return Err((
                StatusCode::BAD_REQUEST,
                CloudApiErrors::UserDoesNotExist.to_string(),
            ))
        }
        Err(err) => {
            error!("Failed to check if user exists: {:?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::DatabaseError.to_string(),
            ));
        }
    }

    // Read session data
    let sessions_key = request
        .action
        .to_session_key(request.email.clone())
        .to_string();

    // Retrieve session data
    let (auth_code, cache_entry) = match sessions_cache.get(&sessions_key) {
        Some(SessionCache::VerifyRegister(mut session)) => {
            // Check if code is correct and not expired
            if check_verification_code(
                &session.verification_code,
                &request.code,
                session.created_at,
            ) {
                // Generate authentication code
                let (auth_code, encrypted_auth_code) = generate_authentication_code();
                session.authentication_code = Some(encrypted_auth_code);

                (auth_code, SessionCache::VerifyRegister(session))
            } else {
                return Err((
                    StatusCode::BAD_REQUEST,
                    CloudApiErrors::InvalidOrExpiredVerificationCode.to_string(),
                ));
            }
        }
        Some(SessionCache::VerifyPasskeyRegister(mut session)) => {
            // Check if code is correct and not expired
            if check_verification_code(
                &session.verification_code,
                &request.code,
                session.created_at,
            ) {
                // Generate authentication code
                let (auth_code, encrypted_auth_code) = generate_authentication_code();
                session.authentication_code = Some(encrypted_auth_code);

                (auth_code, SessionCache::VerifyPasskeyRegister(session))
            } else {
                return Err((
                    StatusCode::BAD_REQUEST,
                    CloudApiErrors::InvalidOrExpiredVerificationCode.to_string(),
                ));
            }
        }
        Some(SessionCache::ResetPassword(mut session)) => {
            // Check if code is correct and not expired
            if check_verification_code(
                &session.verification_code,
                &request.code,
                session.created_at,
            ) {
                // Generate authentication code
                let (auth_code, encrypted_auth_code) = generate_authentication_code();
                session.authentication_code = Some(encrypted_auth_code);

                (auth_code, SessionCache::ResetPassword(session))
            } else {
                return Err((
                    StatusCode::BAD_REQUEST,
                    CloudApiErrors::InvalidOrExpiredVerificationCode.to_string(),
                ));
            }
        }
        Some(SessionCache::ResetPasskey(mut session)) => {
            // Check if code is correct and not expired
            if check_verification_code(
                &session.verification_code,
                &request.code,
                session.created_at,
            ) {
                // Generate authentication code
                let (auth_code, encrypted_auth_code) = generate_authentication_code();
                session.authentication_code = Some(encrypted_auth_code);

                (auth_code, SessionCache::ResetPasskey(session))
            } else {
                return Err((
                    StatusCode::BAD_REQUEST,
                    CloudApiErrors::InvalidOrExpiredVerificationCode.to_string(),
                ));
            }
        }
        _ => {
            return Err((
                StatusCode::BAD_REQUEST,
                CloudApiErrors::InvalidAction.to_string(),
            ));
        }
    };

    // Update cache entry
    sessions_cache.set(sessions_key, cache_entry, None);

    Ok(Json(HttpVerifyCodeResponse {
        verification_code: auth_code,
    }))
}
