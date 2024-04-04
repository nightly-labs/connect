use crate::{
    http::cloud::utils::validate_request,
    structs::{
        cloud::api_cloud_errors::CloudApiErrors,
        session_cache::{ApiSessionsCache, SessionCache, SessionsCacheKey},
    },
};
use axum::{extract::State, http::StatusCode, Json};
use database::{
    db::Db,
    tables::{grafana_users::table_struct::GrafanaUser, utils::get_current_datetime},
};
use garde::Validate;
use log::error;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ts_rs::TS;
use uuid7::uuid7;
use webauthn_rs::prelude::RegisterPublicKeyCredential;
use webauthn_rs::Webauthn;

#[derive(Validate, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HttpRegisterWithPasskeyFinishRequest {
    #[garde(email)]
    pub email: String,
    #[garde(skip)]
    pub credential: RegisterPublicKeyCredential,
}

#[derive(Validate, Clone, Debug, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct HttpRegisterWithPasskeyFinishResponse {}

pub async fn register_with_passkey_finish(
    State(db): State<Arc<Db>>,
    State(web_auth): State<Arc<Webauthn>>,
    State(sessions_cache): State<Arc<ApiSessionsCache>>,
    Json(request): Json<HttpRegisterWithPasskeyFinishRequest>,
) -> Result<Json<HttpRegisterWithPasskeyFinishResponse>, (StatusCode, String)> {
    // Validate request
    validate_request(&request, &())?;

    // Get cache data
    let sessions_key = SessionsCacheKey::PasskeyVerification(request.email.clone()).to_string();
    let session_data = match sessions_cache.get(&sessions_key) {
        Some(SessionCache::VerifyPasskeyRegister(session)) => session,
        _ => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::InternalServerError.to_string(),
            ));
        }
    };

    // Remove leftover session data
    sessions_cache.remove(&sessions_key);

    // Validate passkey register
    let passkey = match web_auth.finish_passkey_registration(
        &request.credential,
        &session_data.passkey_registration_state,
    ) {
        Ok(sk) => sk,
        Err(err) => {
            error!("Failed to finish passkey registration: {:?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::WebAuthnError.to_string(),
            ));
        }
    };

    // Save user to database
    let user_id = uuid7().to_string();
    let grafana_user = GrafanaUser {
        user_id: user_id.clone(),
        email: request.email.clone(),
        // TEMP
        password_hash: serde_json::to_string(&passkey).unwrap(),
        creation_timestamp: get_current_datetime(),
    };
    match db.add_new_user(&grafana_user).await {
        Ok(_) => {
            return Ok(Json(HttpRegisterWithPasskeyFinishResponse {}));
        }
        Err(err) => {
            error!("Failed to create user: {:?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::DatabaseError.to_string(),
            ));
        }
    }
}
