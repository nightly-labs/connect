use crate::{
    http::cloud::utils::{generate_verification_code, validate_request},
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
    test_env::is_test_env,
    utils::get_timestamp_in_milliseconds,
};
use axum::{extract::State, http::StatusCode, Json};
use database::db::Db;
use garde::Validate;
use log::error;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS, Validate)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpResetPasswordStartRequest {
    #[garde(email)]
    pub email: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct HttpResetPasswordStartResponse {}

pub async fn reset_password_start(
    State(db): State<Arc<Db>>,
    State(sessions_cache): State<Arc<ApiSessionsCache>>,
    State(mailer): State<Arc<Mailer>>,
    Json(request): Json<HttpResetPasswordStartRequest>,
) -> Result<Json<HttpResetPasswordStartResponse>, (StatusCode, String)> {
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

    // Save to cache password reset request
    let sessions_key =
        SessionsCacheKey::ResetPasswordVerification(request.email.clone()).to_string();
    // Remove leftover session data
    sessions_cache.remove(&sessions_key);

    // Generate verification code, if not in production use a static code
    let code = if !is_test_env() {
        "123456".to_string()
    } else {
        generate_verification_code()
    };

    sessions_cache.set(
        sessions_key,
        SessionCache::ResetPassword(ResetPasswordVerification {
            email: request.email.clone(),
            verification_code: code.clone(),
            authentication_code: None,
            created_at: get_timestamp_in_milliseconds(),
        }),
        None,
    );

    // Send code via email, only on PROD
    let email_request = SendEmailRequest::ResetPassword(ResetPasswordRequest {
        email: request.email,
        code: code,
    });

    if !is_test_env() {
        if let Some(err) = mailer.handle_email_request(&email_request).error_message {
            error!(
                "Failed to send email: {:?}, email_request: {:?}",
                err, email_request
            );
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::InternalServerError.to_string(),
            ));
        }
    }

    return Ok(Json(HttpResetPasswordStartResponse {}));
}
