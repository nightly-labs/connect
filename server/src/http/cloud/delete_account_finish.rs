use crate::{
    http::cloud::utils::{check_auth_code, validate_request},
    middlewares::auth_middleware::UserId,
    structs::{
        cloud::api_cloud_errors::CloudApiErrors,
        session_cache::{ApiSessionsCache, SessionCache, SessionsCacheKey},
    },
};
use axum::{extract::State, http::StatusCode, Extension, Json};
use database::db::Db;
use garde::Validate;
use log::error;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS, Validate)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpDeleteAccountFinishRequest {
    #[garde(email)]
    pub email: String,
    #[garde(skip)]
    pub auth_code: String,
}

pub async fn delete_account_finish(
    State(db): State<Arc<Db>>,
    State(sessions_cache): State<Arc<ApiSessionsCache>>,
    Extension(user_id): Extension<UserId>,
    Json(request): Json<HttpDeleteAccountFinishRequest>,
) -> Result<Json<()>, (StatusCode, String)> {
    // Validate request
    validate_request(&request, &())?;

    // check if user exists
    match db.get_user_by_user_id(&user_id).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Err((
                StatusCode::BAD_REQUEST,
                CloudApiErrors::UserDoesNotExist.to_string(),
            ));
        }
        Err(err) => {
            error!("Failed to get user: {:?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::DatabaseError.to_string(),
            ));
        }
    };

    // Get session data
    let sessions_key = SessionsCacheKey::DeleteAccount(user_id.clone()).to_string();
    let session_data = match sessions_cache.get(&sessions_key) {
        Some(SessionCache::DeleteAccount(session)) => session,
        _ => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::InternalServerError.to_string(),
            ));
        }
    };

    // Validate auth code
    if !check_auth_code(
        &request.auth_code,
        &session_data.authentication_code,
        session_data.created_at,
    ) {
        return Err((
            StatusCode::BAD_REQUEST,
            CloudApiErrors::InvalidOrExpiredAuthCode.to_string(),
        ));
    }

    // Remove leftover session data
    sessions_cache.remove(&sessions_key);

    // Start transaction to update users privileges
    let mut tx = db.connection_pool.begin().await.map_err(|err| {
        error!("Failed to start transaction: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            CloudApiErrors::DatabaseError.to_string(),
        )
    })?;

    // Deactivate the user
    if let Err(err) = db.deactivate_user(&user_id, &mut tx).await {
        error!("Failed to delete user: {:?}", err);
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            CloudApiErrors::DatabaseError.to_string(),
        ));
    }

    // Delete all user apps
    if let Err(err) = db.deactivate_user_apps(&mut tx, &user_id).await {
        error!("Failed to delete user apps: {:?}", err);
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            CloudApiErrors::DatabaseError.to_string(),
        ));
    }

    // Delete all invites connected to user
    if let Err(err) = db
        .cancel_all_team_invites_containing_email(&mut tx, &request.email, &user_id)
        .await
    {
        error!("Failed to delete team invites: {:?}", err);
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            CloudApiErrors::DatabaseError.to_string(),
        ));
    }

    // Leave all teams
    if let Err(err) = db.remove_inactive_user_from_teams(&mut tx, &user_id).await {
        error!("Failed to leave teams: {:?}", err);
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            CloudApiErrors::DatabaseError.to_string(),
        ));
    }

    // delete privileges
    if let Err(err) = db
        .remove_privileges_for_inactive_teams(&mut tx, &user_id)
        .await
    {
        error!("Failed to leave teams: {:?}", err);
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            CloudApiErrors::DatabaseError.to_string(),
        ));
    }

    // Delete all user teams
    if let Err(err) = db.delete_all_user_teams(&mut tx, &user_id).await {
        error!("Failed to delete user teams: {:?}", err);
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            CloudApiErrors::DatabaseError.to_string(),
        ));
    }

    // Commit transaction
    tx.commit().await.map_err(|err| {
        error!("Failed to commit transaction: {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            CloudApiErrors::DatabaseError.to_string(),
        )
    })?;

    return Ok(Json(()));
}
