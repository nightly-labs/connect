use crate::{
    http::cloud::utils::validate_request,
    structs::{
        cloud::api_cloud_errors::CloudApiErrors,
        session_cache::{ApiSessionsCache, PasskeyVerification, SessionCache, SessionsCacheKey},
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
use webauthn_rs::prelude::{CreationChallengeResponse, Uuid};
use webauthn_rs::Webauthn;

#[derive(Validate, Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct RegisterWithPasskeyStartRequest {
    #[garde(email)]
    pub email: String,
}

pub type RegisterWithPasskeyStartResponse = CreationChallengeResponse;

pub async fn register_with_passkey_start(
    State(db): State<Arc<Db>>,
    State(web_auth): State<Arc<Webauthn>>,
    State(sessions_cache): State<Arc<ApiSessionsCache>>,
    Json(request): Json<RegisterWithPasskeyStartRequest>,
) -> Result<Json<RegisterWithPasskeyStartResponse>, (StatusCode, String)> {
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
    let sessions_key = SessionsCacheKey::PasskeyVerification(request.email.clone()).to_string();

    // Remove leftover session data
    sessions_cache.remove(&sessions_key);

    // Generate challenge
    let temp_user_id = Uuid::new_v4();
    let res =
        web_auth.start_passkey_registration(temp_user_id, &request.email, &request.email, None);

    match res {
        Ok((ccr, reg_state)) => {
            sessions_cache.set(
                sessions_key,
                SessionCache::VerifyPasskeyRegister(PasskeyVerification {
                    email: request.email.clone(),
                    passkey_registration_state: reg_state,
                    created_at: get_timestamp_in_milliseconds(),
                }),
                None,
            );

            return Ok(Json(ccr));
        }
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::WebAuthnError.to_string(),
            ))
        }
    }
}
