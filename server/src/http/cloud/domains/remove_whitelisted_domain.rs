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
pub struct HttpRemoveWhitelistedDomainRequest {
    #[garde(custom(custom_validate_uuid))]
    pub app_id: String,
    #[garde(skip)]
    pub domain_name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct HttpRemoveWhitelistedDomainResponse {}

pub async fn remove_whitelisted_domain(
    State(db): State<Arc<Db>>,
    Extension(user_id): Extension<UserId>,
    Json(request): Json<HttpRemoveWhitelistedDomainRequest>,
) -> Result<Json<HttpRemoveWhitelistedDomainResponse>, (StatusCode, String)> {
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
            if app.deactivated_at != None {
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
    if !app.whitelisted_domains.contains(&domain_name) {
        return Err((
            StatusCode::BAD_REQUEST,
            CloudApiErrors::DomainNotFound.to_string(),
        ));
    }

    let mut tx = db.connection_pool.begin().await.unwrap();

    // Remove domain from whitelisted domains
    if let Err(err) = db
        .remove_whitelisted_domain(&mut tx, &request.app_id, &domain_name)
        .await
    {
        error!(
            "Failed to remove domain from whitelisted domains: {:?}",
            err
        );

        if let Err(err) = tx
            .rollback()
            .await
            .map_err(|err| error!("Failed to rollback transaction: {:?}", err))
        {
            error!("Failed to rollback transaction: {:?}", err);
        }

        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            CloudApiErrors::DatabaseError.to_string(),
        ));
    }

    if let Err(err) = db
        .delete_domain_verification(&mut tx, &domain_name, &request.app_id)
        .await
    {
        error!("Failed to delete domain verification: {:?}", err);

        if let Err(err) = tx
            .rollback()
            .await
            .map_err(|err| error!("Failed to rollback transaction: {:?}", err))
        {
            error!("Failed to rollback transaction: {:?}", err);
        }

        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            CloudApiErrors::DatabaseError.to_string(),
        ));
    }

    if let Err(err) = tx.commit().await {
        error!("Failed to commit transaction: {:?}", err);

        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            CloudApiErrors::DatabaseError.to_string(),
        ));
    }

    return Ok(Json(HttpRemoveWhitelistedDomainResponse {}));
}

#[cfg(feature = "cloud_integration_tests")]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        env::JWT_SECRET,
        http::cloud::register_new_app::HttpRegisterNewAppRequest,
        structs::cloud::cloud_http_endpoints::HttpCloudEndpoint,
        test_utils::test_utils::{
            add_test_app, add_test_team, convert_response, create_test_app, generate_valid_name,
            get_test_app_data, register_and_login_random_user, verify_new_domain,
        },
    };
    use axum::{
        body::Body,
        extract::{ConnectInfo, Request},
        http::Method,
    };
    use database::structs::{
        domain_verification_status::DomainVerificationStatus, whitelisted_domain::WhitelistedDomain,
    };
    use std::net::SocketAddr;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_remove_domain() {
        let test_app = create_test_app(false).await;

        // Register new user
        let (auth_token, _email, _password) = register_and_login_random_user(&test_app).await;

        // Register new team
        let team_name = generate_valid_name();
        let team_id = add_test_team(&team_name, &auth_token, &test_app, false)
            .await
            .unwrap();

        // Register app under the team
        let app_name = generate_valid_name();
        let request = HttpRegisterNewAppRequest {
            team_id: team_id.clone(),
            app_name: app_name.clone(),
        };

        let app_id = add_test_app(&request, &auth_token, &test_app)
            .await
            .unwrap();

        // Add new domain
        let domain_name = "test-domain.com".to_string();
        verify_new_domain(&domain_name, &app_id, &auth_token, &test_app)
            .await
            .unwrap();

        // Get app data and check if domain is whitelisted
        let app_data = get_test_app_data(&team_id, &app_id, &auth_token, &test_app)
            .await
            .unwrap();

        let expected = WhitelistedDomain {
            domain: domain_name.clone(),
            status: DomainVerificationStatus::Verified,
        };

        assert!(app_data.whitelisted_domains.contains(&expected));

        // Remove domain
        let request = HttpRemoveWhitelistedDomainRequest {
            domain_name: domain_name.clone(),
            app_id: app_id.clone(),
        };

        let ip: ConnectInfo<SocketAddr> = ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 8080)));
        let json = serde_json::to_string(&request).unwrap();
        let auth = auth_token.encode(JWT_SECRET()).unwrap();

        let req = Request::builder()
            .method(Method::POST)
            .header("content-type", "application/json")
            .header("authorization", format!("Bearer {auth}"))
            .uri(format!(
                "/cloud/private{}",
                HttpCloudEndpoint::RemoveWhitelistedDomain.to_string()
            ))
            .extension(ip)
            .body(Body::from(json))
            .unwrap();

        // Send request
        let response = test_app.clone().oneshot(req).await.unwrap();
        // Validate response
        let _ = convert_response::<HttpRemoveWhitelistedDomainResponse>(response)
            .await
            .unwrap();

        // Get app data and check if domain was removed
        let app_data = get_test_app_data(&team_id, &app_id, &auth_token, &test_app)
            .await
            .unwrap();

        assert!(!app_data.whitelisted_domains.contains(&expected));
    }
}
