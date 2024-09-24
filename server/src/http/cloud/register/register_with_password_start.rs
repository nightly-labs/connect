use crate::{
    http::cloud::utils::{generate_verification_code, validate_request},
    mailer::{
        mail_requests::{EmailConfirmationRequest, SendEmailRequest},
        mailer::Mailer,
    },
    structs::{
        cloud::api_cloud_errors::CloudApiErrors,
        session_cache::{ApiSessionsCache, RegisterVerification, SessionCache, SessionsCacheKey},
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
pub struct HttpRegisterWithPasswordStartRequest {
    #[garde(email)]
    pub email: String,
    #[garde(skip)]
    pub device: String,
    #[garde(skip)]
    pub browser: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct HttpRegisterWithPasswordStartResponse {}

pub async fn register_with_password_start(
    State(db): State<Arc<Db>>,
    State(sessions_cache): State<Arc<ApiSessionsCache>>,
    State(mailer): State<Arc<Mailer>>,
    Json(request): Json<HttpRegisterWithPasswordStartRequest>,
) -> Result<Json<HttpRegisterWithPasswordStartResponse>, (StatusCode, String)> {
    // Validate request
    validate_request(&request, &())?;

    // Check if user already exists
    match db.get_user_by_email(&request.email).await {
        Ok(Some(_)) => {
            return Err((
                StatusCode::BAD_REQUEST,
                CloudApiErrors::EmailAlreadyExists.to_string(),
            ))
        }
        Ok(None) => {
            // Continue
        }
        Err(err) => {
            error!("Failed to check if user exists: {:?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::DatabaseError.to_string(),
            ));
        }
    }

    // Save to cache register request
    let sessions_key = SessionsCacheKey::RegisterVerification(request.email.clone()).to_string();

    // Remove leftover session data
    sessions_cache.remove(&sessions_key);

    // Generate verification code, if not in production use a static code
    let code = generate_verification_code();

    sessions_cache.set(
        sessions_key,
        SessionCache::VerifyRegister(RegisterVerification {
            email: request.email.clone(),
            verification_code: code.clone(),
            authentication_code: None,
            created_at: get_timestamp_in_milliseconds(),
        }),
        None,
    );

    if !is_test_env() {
        // Send code via email
        let request = SendEmailRequest::EmailConfirmation(EmailConfirmationRequest {
            email: request.email,
            code: code,
            device: request.device.clone(),
            browser: request.browser.clone(),
        });

        // It doesn't matter if this fails
        mailer.handle_email_request(&request);
    }

    return Ok(Json(HttpRegisterWithPasswordStartResponse {}));
}
