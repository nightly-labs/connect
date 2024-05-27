use crate::{
    cloud_state::DnsResolver,
    env::is_env_production,
    http::cloud::utils::{custom_validate_domain_name, custom_validate_uuid},
    middlewares::auth_middleware::UserId,
    structs::cloud::api_cloud_errors::CloudApiErrors,
};
use anyhow::bail;
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
pub struct HttpVerifyDomainFinishRequest {
    #[garde(custom(custom_validate_uuid))]
    pub app_id: String,
    #[garde(skip)]
    pub domain_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct HttpVerifyDomainFinishResponse {}

pub async fn verify_domain_finish(
    State(db): State<Arc<Db>>,
    State(dns_resolver): State<Arc<DnsResolver>>,
    Extension(user_id): Extension<UserId>,
    Json(request): Json<HttpVerifyDomainFinishRequest>,
) -> Result<Json<HttpVerifyDomainFinishResponse>, (StatusCode, String)> {
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

    // Get challenge data
    let domain_verification_challenge = match db
        .get_domain_verification_by_domain_name(&domain_name)
        .await
    {
        Ok(Some(challenge)) => challenge,
        Ok(None) => {
            return Err((
                StatusCode::BAD_REQUEST,
                CloudApiErrors::DomainVerificationNotStarted.to_string(),
            ))
        }
        Err(err) => {
            error!("Failed to get domain verification challenge: {:?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::DatabaseError.to_string(),
            ));
        }
    };

    // Validate the code
    // Attempt to resolve the TXT records for the given domain, only on PROD
    if is_env_production() {
        if let Err(err) = check_verification_code(
            &dns_resolver,
            &domain_name,
            &domain_verification_challenge.code,
        )
        .await
        {
            error!("Failed to verify domain: {:?}, err: {:?}", domain_name, err);
            return Err((
                StatusCode::BAD_REQUEST,
                CloudApiErrors::DomainVerificationFailure.to_string(),
            ));
        }
    }

    // Add domain to whitelist
    let mut tx = db.connection_pool.begin().await.unwrap();

    if let Err(err) = db
        .add_new_whitelisted_domain(&mut tx, &request.app_id, &domain_name)
        .await
    {
        let _ = tx
            .rollback()
            .await
            .map_err(|err| error!("Failed to rollback transaction: {:?}", err));

        error!("Failed to add domain to whitelist: {:?}", err);
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            CloudApiErrors::DatabaseError.to_string(),
        ));
    }

    // Update domain verification entry
    if let Err(err) = db
        .finish_domain_verification(&mut tx, &domain_name, &request.app_id)
        .await
    {
        let _ = tx
            .rollback()
            .await
            .map_err(|err| error!("Failed to rollback transaction: {:?}", err));

        error!("Failed to finish domain verification: {:?}", err);
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            CloudApiErrors::DatabaseError.to_string(),
        ));
    }

    // Commit transaction
    if let Err(err) = tx.commit().await {
        error!("Failed to commit transaction: {:?}", err);

        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            CloudApiErrors::DatabaseError.to_string(),
        ));
    }

    Ok(Json(HttpVerifyDomainFinishResponse {}))
}

async fn check_verification_code(
    dns_resolver: &Arc<DnsResolver>,
    domain_name: &String,
    code: &str,
) -> anyhow::Result<()> {
    match dns_resolver.txt_lookup(domain_name.clone()).await {
        Ok(txt_response) => {
            // Iterate through each TXT record found
            for txt in txt_response.iter() {
                let txt_data = txt.txt_data();
                // Each TXT record can contain multiple strings, so we iterate through them all
                for txt_str in txt_data {
                    let txt_str = std::str::from_utf8(txt_str).unwrap();
                    // Check if the verification code is present
                    if txt_str.contains(&code) {
                        return Ok(());
                    }
                }
            }
            bail!("Verification code not found in TXT records");
        }
        Err(_) => {
            bail!("Failed to resolve TXT records");
        }
    }
}
