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
use webauthn_rs::prelude::PublicKeyCredential;
use webauthn_rs::Webauthn;

#[derive(Validate, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpDeletePasskeyRequest {
    #[garde(skip)]
    pub passkey_id: String,
    #[garde(skip)]
    pub passkey_credential: PublicKeyCredential,
}

pub async fn delete_passkey(
    State(db): State<Arc<Db>>,
    State(web_auth): State<Arc<Webauthn>>,
    State(sessions_cache): State<Arc<ApiSessionsCache>>,
    Extension(user_id): Extension<UserId>,
    Json(payload): Json<HttpDeletePasskeyRequest>,
) -> Result<(), (StatusCode, String)> {
    // Get cache data
    let sessions_key = SessionsCacheKey::Passkey2FA(user_id.clone()).to_string();
    let session_data = match sessions_cache.get(&sessions_key) {
        Some(SessionCache::Passkey2FA(session)) => session,
        _ => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::InternalServerError.to_string(),
            ));
        }
    };

    // Remove leftover session data
    sessions_cache.remove(&sessions_key);

    // Finish passkey authentication
    if let Err(err) = web_auth.finish_passkey_authentication(
        &payload.passkey_credential,
        &session_data.passkey_verification_state,
    ) {
        warn!(
            "Failed to finish passkey authentication: {:?}, user_id: {}",
            err, user_id
        );
        return Err((
            StatusCode::BAD_REQUEST,
            CloudApiErrors::InvalidPasskeyCredential.to_string(),
        ));
    }

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

    // Get user passkeys
    let mut passkeys = match user_data.passkeys {
        Some(passkey) => passkey,
        None => {
            return Err((
                StatusCode::BAD_REQUEST,
                CloudApiErrors::UserDoesNotHavePasskey.to_string(),
            ));
        }
    };

    // Remove passkey
    match passkeys
        .iter()
        .position(|x| x.cred_id().to_string() == payload.passkey_id)
    {
        Some(index) => {
            // Remove passkey
            passkeys.remove(index);
        }
        None => {
            return Err((
                StatusCode::BAD_REQUEST,
                CloudApiErrors::PasskeyDoesNotExist.to_string(),
            ))
        }
    }

    // Update user passkeys in database
    if let Err(err) = db.update_passkeys(&user_id, &passkeys).await {
        error!(
            "Failed to update user passkeys: {:?}, user_id: {}",
            err, user_id
        );

        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            CloudApiErrors::DatabaseError.to_string(),
        ));
    }

    return Ok(());
}
