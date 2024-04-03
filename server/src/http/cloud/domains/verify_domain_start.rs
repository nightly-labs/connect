use crate::{
    http::cloud::utils::{custom_validate_domain_name, custom_validate_uuid},
    middlewares::auth_middleware::UserId,
    structs::{
        cloud::api_cloud_errors::CloudApiErrors,
        session_cache::{ApiSessionsCache, DomainVerification, SessionCache, SessionsCacheKey},
    },
    utils::get_timestamp_in_milliseconds,
};
use axum::{extract::State, http::StatusCode, Extension, Json};
use database::{db::Db, structs::privilege_level::PrivilegeLevel};
use garde::Validate;
use log::error;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS, Validate)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpVerifyDomainStartRequest {
    #[garde(custom(custom_validate_uuid))]
    pub app_id: String,
    #[garde(skip)]
    pub domain_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct HttpVerifyDomainStartResponse {
    pub code: String,
}

pub async fn verify_domain_start(
    State(db): State<Arc<Db>>,
    State(sessions_cache): State<Arc<ApiSessionsCache>>,
    Extension(user_id): Extension<UserId>,
    Json(request): Json<HttpVerifyDomainStartRequest>,
) -> Result<Json<HttpVerifyDomainStartResponse>, (StatusCode, String)> {
    // Validate domain name
    let domain_name = custom_validate_domain_name(&request.domain_name).map_err(|e| {
        error!("Failed to validate domain name: {:?}", e);
        (
            StatusCode::BAD_REQUEST,
            CloudApiErrors::InvalidDomainName.to_string(),
        )
    })?;

    // Check if app exists and get data
    let app = match db.get_registered_app_by_app_id(&request.app_id).await {
        Ok(Some(app)) => app,
        Ok(None) => {
            return Err((
                StatusCode::BAD_REQUEST,
                CloudApiErrors::AppDoesNotExist.to_string(),
            ))
        }
        Err(err) => {
            error!("Failed to check if app exists: {:?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::DatabaseError.to_string(),
            ));
        }
    };

    // Check if user has sufficient permissions
    match db
        .get_privilege_by_user_id_and_app_id(&user_id, &request.app_id)
        .await
    {
        Ok(Some(privilege)) => {
            // User needs to have admin privileges
            if privilege.privilege_level != PrivilegeLevel::Admin {
                return Err((
                    StatusCode::BAD_REQUEST,
                    CloudApiErrors::InsufficientPermissions.to_string(),
                ));
            }
        }
        Ok(None) => {
            return Err((
                StatusCode::BAD_REQUEST,
                CloudApiErrors::InsufficientPermissions.to_string(),
            ))
        }
        Err(err) => {
            error!(
                "Failed to check if user has sufficient permissions: {:?}",
                err
            );
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::DatabaseError.to_string(),
            ));
        }
    }

    // Check if domain is already verified
    if app.whitelisted_domains.contains(&domain_name) {
        return Err((
            StatusCode::BAD_REQUEST,
            CloudApiErrors::DomainAlreadyVerified.to_string(),
        ));
    }

    // Generate verification code
    let verification_code =
        format!("TXT Nc verification code {}", uuid7::uuid7().to_string()).to_string();

    // Save to cache
    let sessions_key = SessionsCacheKey::DomainVerification(domain_name.clone()).to_string();
    // Remove leftover session data
    sessions_cache.remove(&sessions_key);

    sessions_cache.set(
        sessions_key,
        SessionCache::VerifyDomain(DomainVerification {
            domain_name: domain_name.clone(),
            code: verification_code.clone(),
            created_at: get_timestamp_in_milliseconds(),
        }),
        None,
    );

    Ok(Json(HttpVerifyDomainStartResponse {
        code: verification_code,
    }))
}
