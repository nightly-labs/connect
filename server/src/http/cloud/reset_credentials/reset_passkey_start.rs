use crate::{
    env::is_env_production,
    http::cloud::utils::{generate_verification_code, validate_request},
    mailer::{
        mail_requests::{EmailConfirmationRequest, SendEmailRequest},
        mailer::Mailer,
    },
    structs::{
        cloud::api_cloud_errors::CloudApiErrors,
        session_cache::{ApiSessionsCache, PasskeyVerification, SessionCache, SessionsCacheKey},
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
use webauthn_rs::prelude::{CreationChallengeResponse, Uuid};
use webauthn_rs::Webauthn;

#[derive(Validate, Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpResetPasskeyStartRequest {
    #[garde(email)]
    pub email: String,
}

pub type HttpResetPasskeyStartResponse = CreationChallengeResponse;

pub async fn reset_passkey_start(
    State(db): State<Arc<Db>>,
    State(web_auth): State<Arc<Webauthn>>,
    State(mailer): State<Arc<Mailer>>,
    State(sessions_cache): State<Arc<ApiSessionsCache>>,
    Json(request): Json<HttpResetPasskeyStartRequest>,
) -> Result<Json<HttpResetPasskeyStartResponse>, (StatusCode, String)> {
    // Validate request
    validate_request(&request, &())?;

    // Check if user already exists
    match db.get_user_by_email(&request.email).await {
        Ok(Some(user_data)) => {
            // Check if user has a passkey
            if user_data.passkeys.is_none() {
                return Err((
                    StatusCode::BAD_REQUEST,
                    CloudApiErrors::UserDoesNotHavePasskey.to_string(),
                ));
            }
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

    // Save to cache register request
    let sessions_key = SessionsCacheKey::PasskeyVerification(request.email.clone()).to_string();

    // Remove leftover session data
    sessions_cache.remove(&sessions_key);

    // Generate challenge
    let temp_user_id = Uuid::new_v4();
    let res =
        web_auth.start_passkey_registration(temp_user_id, &request.email, &request.email, None);

    let (ccr, reg_state) = match res {
        Ok((ccr, reg_state)) => (ccr, reg_state),
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::WebAuthnError.to_string(),
            ))
        }
    };

    // Generate verification code, if not in production use a static code
    let code = if !is_env_production() {
        "123456".to_string()
    } else {
        generate_verification_code()
    };

    // Send email with code
    let email_request = SendEmailRequest::EmailConfirmation(EmailConfirmationRequest {
        email: request.email.clone(),
        code: code.clone(),
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

    // Save the challenge to the cache
    sessions_cache.set(
        sessions_key,
        SessionCache::VerifyPasskeyRegister(PasskeyVerification {
            email: request.email.clone(),
            passkey_registration_state: reg_state,
            verification_code: code,
            authentication_code: None,
            created_at: get_timestamp_in_milliseconds(),
        }),
        None,
    );

    return Ok(Json(ccr));
}
