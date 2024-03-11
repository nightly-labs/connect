use crate::{
    auth::auth_middleware::UserId, statics::USERS_AMOUNT_LIMIT_PER_TEAM,
    structs::api_cloud_errors::CloudApiErrors, utils::validate_request,
};
use axum::{extract::State, http::StatusCode, Extension, Json};
use database::db::Db;
use log::error;
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, sync::Arc};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct HttpGetUserJoinedTeamsRequest {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct HttpGetUserJoinedTeamsResponse {
    // pub teams_map:
}

pub async fn add_user_to_team(
    State(db): State<Option<Arc<Db>>>,
    Extension(user_id): Extension<UserId>,
    Json(request): Json<HttpGetUserJoinedTeamsRequest>,
) -> Result<Json<HttpGetUserJoinedTeamsResponse>, (StatusCode, String)> {
    // Db connection has already been checked in the middleware
    let db = db.as_ref().ok_or((
        StatusCode::INTERNAL_SERVER_ERROR,
        CloudApiErrors::CloudFeatureDisabled.to_string(),
    ))?;

    // Check if user already belongs to the team
    match db.get_teams_and_apps_membership_by_user_id(&user_id).await {
        Ok(teams) => {}
        Err(err) => {
            error!(
                "Failed to get teams and apps membership by user id: {:?}",
                err
            );
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::DatabaseError.to_string(),
            ));
        }
    }
}
