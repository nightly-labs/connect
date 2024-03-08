use axum::{extract::State, http::StatusCode, Json};
use database::{
    db::Db,
    tables::{grafana_users::table_struct::GrafanaUser, utils::get_current_datetime},
};
use garde::Validate;
use log::error;
use pwhash::bcrypt;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ts_rs::TS;
use uuid7::uuid7;

use crate::{env::NONCE, structs::api_cloud_errors::CloudApiErrors, utils::validate_request};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS, Validate)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpRegisterWithPasswordRequest {
    #[garde(email)]
    pub email: String,
    #[garde(ascii, length(min = 6, max = 30))]
    pub password: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct HttpRegisterWithPasswordResponse {
    pub user_id: String,
}

pub async fn register_with_password(
    State(db): State<Option<Arc<Db>>>,
    Json(request): Json<HttpRegisterWithPasswordRequest>,
) -> Result<Json<HttpRegisterWithPasswordResponse>, (StatusCode, String)> {
    // Db connection has already been checked in the middleware
    let db = db.as_ref().ok_or((
        StatusCode::INTERNAL_SERVER_ERROR,
        CloudApiErrors::CloudFeatureDisabled.to_string(),
    ))?;

    // Validate request
    validate_request(&request, &())?;

    // Check if user already exists
    if let Ok(_) = db.get_user_by_email(&request.email).await {
        return Err((
            StatusCode::BAD_REQUEST,
            CloudApiErrors::EmailAlreadyExists.to_string(),
        ));
    }

    let hashed_password = bcrypt::hash(format!("{}_{}", NONCE(), request.password.clone()))
        .map_err(|e| {
            error!("Failed to hash password: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::InternalServerError.to_string(),
            )
        })?;

    // Create new user
    let user_id = uuid7().to_string();
    let grafana_user = GrafanaUser {
        user_id: user_id.clone(),
        email: request.email.clone(),
        password_hash: hashed_password,
        creation_timestamp: get_current_datetime(),
    };

    if let Err(err) = db.add_new_user(&grafana_user).await {
        error!("Failed to create user: {:?}", err);
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            CloudApiErrors::DatabaseError.to_string(),
        ));
    }

    return Ok(Json(HttpRegisterWithPasswordResponse { user_id }));
}
