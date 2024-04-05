use crate::{
    middlewares::auth_middleware::UserId,
    structs::{
        cloud::api_cloud_errors::CloudApiErrors,
        session_cache::{AddPasskeyVerification, ApiSessionsCache, SessionCache, SessionsCacheKey},
    },
    utils::get_timestamp_in_milliseconds,
};
use axum::{extract::State, http::StatusCode, Extension, Json};
use database::db::Db;
use garde::Validate;
use log::error;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ts_rs::TS;
use webauthn_rs::{
    prelude::{CreationChallengeResponse, Uuid},
    Webauthn,
};

#[derive(Validate, Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct HttpAddPasskeyStartRequest {
    #[garde(email)]
    pub email: String,
}

pub type HttpAddPasskeyStartResponse = CreationChallengeResponse;

pub async fn add_passkey_start(
    State(db): State<Arc<Db>>,
    State(web_auth): State<Arc<Webauthn>>,
    State(sessions_cache): State<Arc<ApiSessionsCache>>,
    Extension(user_id): Extension<UserId>,
    Json(request): Json<HttpAddPasskeyStartRequest>,
) -> Result<Json<HttpAddPasskeyStartResponse>, (StatusCode, String)> {
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

    // Save to cache passkey challenge request
    let sessions_key = SessionsCacheKey::AddPasskey(user_id.clone()).to_string();

    // Remove leftover session data
    sessions_cache.remove(&sessions_key);

    // Generate challenge
    let temp_user_id = Uuid::new_v4();
    let res =
        web_auth.start_passkey_registration(temp_user_id, &user_data.email, &user_data.email, None);

    let (ccr, reg_state) = match res {
        Ok((ccr, reg_state)) => (ccr, reg_state),
        Err(_) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::WebAuthnError.to_string(),
            ))
        }
    };

    // Save the challenge to the cache
    sessions_cache.set(
        sessions_key,
        SessionCache::VerifyAddPasskey(AddPasskeyVerification {
            email: request.email.clone(),
            passkey_registration_state: reg_state,
            created_at: get_timestamp_in_milliseconds(),
        }),
        None,
    );

    return Ok(Json(ccr));
}
