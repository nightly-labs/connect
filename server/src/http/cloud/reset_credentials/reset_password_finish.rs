use crate::{
    env::is_env_production,
    http::cloud::utils::{custom_validate_verification_code, validate_request},
    structs::{
        cloud::api_cloud_errors::CloudApiErrors,
        session_cache::{ApiSessionsCache, SessionCache, SessionsCacheKey},
    },
};
use axum::{extract::State, http::StatusCode, Json};
use database::db::Db;
use garde::Validate;
use log::error;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS, Validate)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpResetPasswordFinishRequest {
    #[garde(email)]
    pub email: String,
    #[garde(custom(custom_validate_verification_code))]
    pub code: String,
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

    // Remove leftover session data
    sessions_cache.remove(&sessions_key);

    // validate code only on production
    if is_env_production() {
        // Validate the code
        if session_data.code != request.code {
            return Err((
                StatusCode::BAD_REQUEST,
                CloudApiErrors::InvalidVerificationCode.to_string(),
            ));
        }
    }

    // Update user password
    if let Err(err) = db
        .set_new_password(&session_data.email, &session_data.hashed_new_password)
        .await
    {
        error!("Failed to create user: {:?}", err);
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            CloudApiErrors::DatabaseError.to_string(),
        ));
    }

    return Ok(Json(HttpResetPasswordFinishResponse {}));
}

#[cfg(feature = "cloud_db_tests")]
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
            new_password: new_password.to_string(),
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
            // Random valid code for testing
            code: "123456".to_string(),
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
