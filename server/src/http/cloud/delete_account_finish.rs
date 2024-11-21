use crate::{
    env::is_env_production,
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
use openapi::apis::configuration::Configuration;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ts_rs::TS;

use super::grafana_utils::delete_user_account::handle_grafana_delete_user_account;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS, Validate)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpDeleteAccountFinishRequest {
    #[garde(skip)]
    pub auth_code: String,
}

pub async fn delete_account_finish(
    State(db): State<Arc<Db>>,
    State(sessions_cache): State<Arc<ApiSessionsCache>>,
    State(grafana_conf): State<Arc<Configuration>>,
    Extension(user_id): Extension<UserId>,
    Json(request): Json<HttpDeleteAccountFinishRequest>,
) -> Result<Json<()>, (StatusCode, String)> {
    // Validate request
    validate_request(&request, &())?;

    // check if user exists
    let user = match db.get_user_by_user_id(&user_id).await {
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
    let sessions_key = SessionsCacheKey::DeleteAccount(user.email.clone()).to_string();
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

    // Grafana, delete teams, apps and user
    if is_env_production() {
        let mut owned_team_grafana_ids = Vec::new();
        let mut non_owned_team_grafana_ids = Vec::new();

        let teams = match db
            .get_joined_teams_by_user_id(&user_id)
            .await
            .map_err(|err| {
                error!("Failed to get user teams: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    CloudApiErrors::DatabaseError.to_string(),
                )
            }) {
            Ok(joined_teams) => joined_teams,
            Err(err) => {
                error!("Failed to get user teams: {:?}", err);
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    CloudApiErrors::DatabaseError.to_string(),
                ));
            }
        };

        for (team, _, _, _) in teams {
            if let Some(grafana_id) = team.grafana_id {
                if team.team_admin_id == user_id {
                    owned_team_grafana_ids.push(grafana_id);
                } else {
                    non_owned_team_grafana_ids.push(grafana_id);
                }
            }
        }

        if let Err(err) = handle_grafana_delete_user_account(
            &grafana_conf,
            &owned_team_grafana_ids,
            &non_owned_team_grafana_ids,
            &user.email,
        )
        .await
        {
            error!("Failed to delete account in grafana: {:?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::GrafanaError.to_string(),
            ));
        };
    }

    // Delete all invites connected to user
    if let Err(err) = db
        .cancel_all_team_invites_containing_email(&mut tx, &user.email, &user_id)
        .await
    {
        error!("Failed to delete team invites: {:?}", err);
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

    // Deactivate the user
    if let Err(err) = db.deactivate_user(&user_id, &mut tx).await {
        error!("Failed to delete user: {:?}", err);
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
