use crate::{
    env::NONCE,
    http::cloud::utils::{check_auth_code, custom_validate_new_password, validate_request},
    structs::{
        cloud::api_cloud_errors::CloudApiErrors,
        session_cache::{ApiSessionsCache, SessionCache, SessionsCacheKey},
    },
};
use axum::{extract::State, http::StatusCode, Json};
use database::db::Db;
use garde::Validate;
use log::error;
use pwhash::bcrypt;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS, Validate)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpResetPasswordFinishRequest {
    #[garde(email)]
    pub email: String,
    #[garde(custom(custom_validate_new_password))]
    pub new_password: String,
    #[garde(skip)]
    pub auth_code: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct HttpResetPasswordFinishResponse {}

pub async fn reset_password_finish(
    State(db): State<Arc<Db>>,
    State(sessions_cache): State<Arc<ApiSessionsCache>>,
    Json(request): Json<HttpResetPasswordFinishRequest>,
) -> Result<Json<HttpResetPasswordFinishResponse>, (StatusCode, String)> {
    // Validate request
    validate_request(&request, &())?;

    // Get session data
    let sessions_key =
        SessionsCacheKey::ResetPasswordVerification(request.email.clone()).to_string();
    let session_data = match sessions_cache.get(&sessions_key) {
        Some(SessionCache::ResetPassword(session)) => session,
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

    let hashed_password = bcrypt::hash(format!("{}_{}", NONCE(), request.new_password.clone()))
        .map_err(|e| {
            error!("Failed to hash password: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::InternalServerError.to_string(),
            )
        })?;

    // Remove leftover session data
    sessions_cache.remove(&sessions_key);

    // Update user password
    if let Err(err) = db
        .set_new_password(&session_data.email, &hashed_password)
        .await
    {
        error!("Failed to set new password: {:?}", err);
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            CloudApiErrors::DatabaseError.to_string(),
        ));
    }

    return Ok(Json(HttpResetPasswordFinishResponse {}));
}

#[cfg(feature = "cloud_integration_tests")]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        http::cloud::{
            login::login_with_password::{HttpLoginRequest, HttpLoginResponse},
            reset_credentials::reset_password_start::{
                HttpResetPasswordStartRequest, HttpResetPasswordStartResponse,
            },
        },
        structs::cloud::cloud_http_endpoints::HttpCloudEndpoint,
        test_utils::test_utils::{
            convert_response, create_test_app, register_and_login_random_user,
        },
    };
    use axum::{
        body::Body,
        extract::ConnectInfo,
        http::{Method, Request},
    };
    use std::net::SocketAddr;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_reset_password() {
        let test_app = create_test_app(false).await;

        // Register new user
        let (_auth_token, email, _password) = register_and_login_random_user(&test_app).await;

        // Start password reset
        let new_password = "NewValidPassword123".to_string();

        let password_reset_start_payload = HttpResetPasswordStartRequest {
            email: email.to_string(),
            device: "device".to_string(),
            browser: "browser".to_string(),
        };

        let ip: ConnectInfo<SocketAddr> = ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 8080)));
        let json = serde_json::to_string(&password_reset_start_payload).unwrap();

        let req = Request::builder()
            .method(Method::POST)
            .header("content-type", "application/json")
            .uri(format!(
                "/cloud/public{}",
                HttpCloudEndpoint::ResetPasswordStart.to_string()
            ))
            .extension(ip)
            .body(Body::from(json))
            .unwrap();

        // Send request
        let register_response = test_app.clone().oneshot(req).await.unwrap();
        // Validate response
        convert_response::<HttpResetPasswordStartResponse>(register_response)
            .await
            .unwrap();

        // Validate new password change
        let verify_register_payload = HttpResetPasswordFinishRequest {
            email: email.to_string(),
            new_password: new_password.to_string(),
            // Random code for testing
            auth_code: "123456789".to_string(),
        };

        let ip: ConnectInfo<SocketAddr> = ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 8080)));
        let json = serde_json::to_string(&verify_register_payload).unwrap();

        let req = Request::builder()
            .method(Method::POST)
            .header("content-type", "application/json")
            .uri(format!(
                "/cloud/public{}",
                HttpCloudEndpoint::ResetPasswordFinish.to_string()
            ))
            .extension(ip)
            .body(Body::from(json))
            .unwrap();

        // send request to app and get response
        let verify_register_response = test_app.clone().oneshot(req).await.unwrap();
        assert_eq!(verify_register_response.status(), StatusCode::OK);

        // Log in using new password
        // Login user
        let login_payload = HttpLoginRequest {
            email: email.to_string(),
            password: new_password.to_string(),
            enforce_ip: false,
        };

        let json = serde_json::to_string(&login_payload).unwrap();
        let req = Request::builder()
            .method(Method::POST)
            .header("content-type", "application/json")
            .uri(format!(
                "/cloud/public{}",
                HttpCloudEndpoint::LoginWithPassword.to_string()
            ))
            .extension(ip)
            .body(Body::from(json))
            .unwrap();

        // Send request
        let login_response = test_app.clone().oneshot(req).await.unwrap();
        // Validate response
        convert_response::<HttpLoginResponse>(login_response)
            .await
            .unwrap();
    }
}
