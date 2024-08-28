use crate::{
    middlewares::auth_middleware::UserId,
    structs::{
        cloud::api_cloud_errors::CloudApiErrors,
        session_cache::{ApiSessionsCache, SessionCache, SessionsCacheKey},
    },
};
use axum::{extract::State, http::StatusCode, Extension, Json};
use database::db::Db;
use garde::Validate;
use log::{error, warn};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ts_rs::TS;
use webauthn_rs::{prelude::RegisterPublicKeyCredential, Webauthn};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HttpAddPasskeyFinishRequest {
    pub credential: RegisterPublicKeyCredential,
}

#[derive(Validate, Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct HttpAddPasskeyFinishResponse {}

pub async fn add_passkey_finish(
    State(db): State<Arc<Db>>,
    State(web_auth): State<Arc<Webauthn>>,
    State(sessions_cache): State<Arc<ApiSessionsCache>>,
    Extension(user_id): Extension<UserId>,
    Json(request): Json<HttpAddPasskeyFinishRequest>,
) -> Result<Json<HttpAddPasskeyFinishResponse>, (StatusCode, String)> {
    // Get cache data
    let sessions_key = SessionsCacheKey::AddPasskey(user_id.clone()).to_string();
    let session_data = match sessions_cache.get(&sessions_key) {
        Some(SessionCache::VerifyAddPasskey(session)) => session,
        _ => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::InternalServerError.to_string(),
            ));
        }
    };

    // Remove leftover session data
    sessions_cache.remove(&sessions_key);

    // Validate new passkey registration
    let passkey = match web_auth.finish_passkey_registration(
        &request.credential,
        &session_data.passkey_registration_state,
    ) {
        Ok(sk) => sk,
        Err(err) => {
            warn!(
                "Failed to finish adding new passkey: {:?}, user_id: {}",
                err, &user_id
            );
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::WebAuthnError.to_string(),
            ));
        }
    };

    // Validate new passkey
    // Get user data
    let user_data = match db.get_user_by_user_id(&user_id).await {
        Ok(Some(user_data)) => user_data,
        Ok(None) => {
            return Err((
                StatusCode::BAD_REQUEST,
                CloudApiErrors::UserDoesNotExist.to_string(),
            ));
        }
        Err(err) => {
            error!("Failed to get user data: {:?}, user_id: {}", err, &user_id);
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
            vec![]
        }
    };

    // Add new passkey
    passkeys.push(passkey);

    // Update passkeys in database
    if let Err(err) = db.update_passkeys(&user_data.email, &passkeys).await {
        error!(
            "Failed to update user passkeys: {:?}, user_email: {}",
            err, &user_data.email
        );
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            CloudApiErrors::DatabaseError.to_string(),
        ));
    }

    return Ok(Json(HttpAddPasskeyFinishResponse {}));
}
