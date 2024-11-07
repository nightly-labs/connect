use crate::{
    mailer::{
        mail_requests::{EmailConfirmationRequest, SendEmailRequest},
        mailer::Mailer,
    },
    middlewares::auth_middleware::UserId,
    structs::{
        cloud::api_cloud_errors::CloudApiErrors,
        session_cache::{
            ApiSessionsCache, DeleteAccountVerification, SessionCache, SessionsCacheKey,
        },
    },
    test_env::is_test_env,
    utils::get_timestamp_in_milliseconds,
};
use axum::{extract::State, http::StatusCode, Extension, Json};
use database::db::Db;
use garde::Validate;
use log::error;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ts_rs::TS;

use super::utils::generate_verification_code;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS, Validate)]
#[ts(export)]
pub struct HttpDeleteAccountStartRequest {
    #[garde(alphanumeric)]
    pub device: String,
    #[garde(alphanumeric)]
    pub browser: String,
}
pub async fn delete_account_start(
    State(db): State<Arc<Db>>,
    State(mailer): State<Arc<Mailer>>,
    State(sessions_cache): State<Arc<ApiSessionsCache>>,
    Extension(user_id): Extension<UserId>,
    Json(request): Json<HttpDeleteAccountStartRequest>,
) -> Result<Json<()>, (StatusCode, String)> {
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

    // Save to cache dlete account challenge request
    let sessions_key = SessionsCacheKey::DeleteAccount(user_id.clone()).to_string();

    // Remove leftover session data
    sessions_cache.remove(&sessions_key);

    // Generate verification code, if not in production use a static code
    let verification_code = generate_verification_code();

    // Save the challenge to the cache
    sessions_cache.set(
        sessions_key,
        SessionCache::DeleteAccount(DeleteAccountVerification {
            user_id,
            verification_code: verification_code.clone(),
            authentication_code: None,
            created_at: get_timestamp_in_milliseconds(),
        }),
        None,
    );

    if !is_test_env() {
        // Send code via email
        let request = SendEmailRequest::EmailConfirmation(EmailConfirmationRequest {
            email: user_data.email,
            code: verification_code,
            device: request.device.clone(),
            browser: request.browser.clone(),
        });

        // It doesn't matter if this fails
        mailer.handle_email_request(&request);
    }

    return Ok(Json(()));
}
