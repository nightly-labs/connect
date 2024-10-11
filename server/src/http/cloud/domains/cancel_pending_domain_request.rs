use crate::{
    http::cloud::utils::{custom_validate_domain_name, custom_validate_uuid},
    middlewares::auth_middleware::UserId,
    structs::cloud::api_cloud_errors::CloudApiErrors,
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
pub struct HttpCancelPendingDomainVerificationRequest {
    #[garde(custom(custom_validate_uuid))]
    pub app_id: String,
    #[garde(skip)]
    pub domain_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct HttpCancelPendingDomainVerificationResponse {}

pub async fn cancel_pending_domain_request(
    State(db): State<Arc<Db>>,
    Extension(user_id): Extension<UserId>,
    Json(request): Json<HttpCancelPendingDomainVerificationRequest>,
) -> Result<Json<HttpCancelPendingDomainVerificationResponse>, (StatusCode, String)> {
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
        Ok(Some(app)) => {
            if app.active == false {
                return Err((
                    StatusCode::BAD_REQUEST,
                    CloudApiErrors::AppDoesNotExist.to_string(),
                ));
            }
            app
        }
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

    // Check if domain is whitelisted
    if app.whitelisted_domains.contains(&domain_name) {
        return Err((
            StatusCode::BAD_REQUEST,
            CloudApiErrors::DomainAlreadyVerified.to_string(),
        ));
    }

    // Check if there is a pending domain verification
    match db
        .get_pending_domain_verification_by_domain_name_and_app_id(&domain_name, &request.app_id)
        .await
    {
        Ok(Some(_)) => {
            // Cancel domain verification
            if let Err(err) = db
                .cancel_domain_verification(&domain_name, &request.app_id)
                .await
            {
                error!("Failed to cancel domain verification: {:?}", err);
                return Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    CloudApiErrors::DatabaseError.to_string(),
                ));
            }
        }
        Ok(None) => {
            return Err((
                StatusCode::BAD_REQUEST,
                CloudApiErrors::NoPendingDomainVerification.to_string(),
            ));
        }
        Err(err) => {
            error!("Failed to check if domain verification exists: {:?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::DatabaseError.to_string(),
            ));
        }
    }

    return Ok(Json(HttpCancelPendingDomainVerificationResponse {}));
}
