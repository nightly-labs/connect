use crate::{
    env::{is_env_production, NONCE},
    http::cloud::utils::{
        custom_validate_new_password, generate_verification_code, validate_request,
    },
    mailer::{
        mail_requests::{ResetPasswordRequest, SendEmailRequest},
        mailer::Mailer,
    },
    structs::{
        cloud::api_cloud_errors::CloudApiErrors,
        session_cache::{
            ApiSessionsCache, ResetPasswordVerification, SessionCache, SessionsCacheKey,
        },
    },
    utils::get_timestamp_in_milliseconds,
};
use axum::{extract::State, http::StatusCode, Json};
use database::db::Db;
use garde::Validate;
use log::error;
use pwhash::bcrypt;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS, Validate)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpResetPasswordRequest {
    #[garde(email)]
    pub email: String,
    #[garde(custom(custom_validate_new_password))]
    pub new_password: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct HttpResetPasswordResponse {}

pub async fn reset_password_start(
    State(db): State<Arc<Db>>,
    State(sessions_cache): State<Arc<ApiSessionsCache>>,
    State(mailer): State<Arc<Mailer>>,
    Json(request): Json<HttpResetPasswordRequest>,
) -> Result<Json<HttpResetPasswordResponse>, (StatusCode, String)> {
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

    let hashed_password = bcrypt::hash(format!("{}_{}", NONCE(), request.new_password.clone()))
        .map_err(|e| {
            error!("Failed to hash password: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::InternalServerError.to_string(),
            )
        })?;

    // Save to cache password reset request
    let sessions_key =
        SessionsCacheKey::ResetPasswordVerification(request.email.clone()).to_string();
    // Remove leftover session data
    sessions_cache.remove(&sessions_key);

    let code = generate_verification_code();

    sessions_cache.set(
        sessions_key,
        SessionCache::ResetPassword(ResetPasswordVerification {
            email: request.email.clone(),
            hashed_new_password: hashed_password,
            code: code.clone(),
            created_at: get_timestamp_in_milliseconds(),
        }),
        None,
    );

    // Send code via email, only on PROD
    if is_env_production() {
        let request = SendEmailRequest::ResetPassword(ResetPasswordRequest {
            email: request.email,
            code: code,
        });

        match mailer.handle_email_request(&request).error_message {
            Some(err) => {
                error!("Failed to send email: {:?}, request: {:?}", err, request);
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    CloudApiErrors::InternalServerError.to_string(),
                ));
            }
            None => {
                return Ok(Json(HttpResetPasswordResponse {}));
            }
        }
    }

    Ok(Json(HttpResetPasswordResponse {}))
}
