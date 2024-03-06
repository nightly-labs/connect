use axum::{extract::State, http::StatusCode, Json};
use database::{
    db::Db,
    tables::{grafana_users::table_struct::GrafanaUser, utils::get_current_datetime},
};
use log::error;
use pwhash::bcrypt;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ts_rs::TS;
use uuid7::uuid7;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpRegisterNewUserRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct HttpRegisterNewUserResponse {
    pub user_id: String,
}

pub async fn register_new_user(
    State(db): State<Option<Arc<Db>>>,
    Json(request): Json<HttpRegisterNewUserRequest>,
) -> Result<Json<HttpRegisterNewUserResponse>, (StatusCode, String)> {
    // Db connection has already been checked in the middleware
    let db = db.as_ref().ok_or((
        StatusCode::INTERNAL_SERVER_ERROR,
        "Failed to get database connection".to_string(),
    ))?;

    // Check if user already exists
    if let Ok(_) = db.get_user_by_email(&request.email).await {
        return Err((
            StatusCode::BAD_REQUEST,
            "User with this email already exists".to_string(),
        ));
    }

    let hashed_password = bcrypt::hash(request.password.clone())
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

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
            "Failed to create user".to_string(),
        ));
    }

    return Ok(Json(HttpRegisterNewUserResponse { user_id }));
}
