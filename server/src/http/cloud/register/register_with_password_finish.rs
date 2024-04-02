use crate::{
    env::is_env_production,
    http::cloud::utils::{custom_validate_verification_code, validate_request},
    structs::{
        cloud::api_cloud_errors::CloudApiErrors,
        session_cache::{ApiSessionsCache, SessionCache, SessionsCacheKey},
    },
};
use axum::{extract::State, http::StatusCode, Json};
use database::{
    db::Db,
    tables::{grafana_users::table_struct::GrafanaUser, utils::get_current_datetime},
};
use garde::Validate;
use log::error;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ts_rs::TS;
use uuid7::uuid7;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS, Validate)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpVerifyRegisterWithPasswordRequest {
    #[garde(email)]
    pub email: String,
    #[garde(custom(custom_validate_verification_code))]
    pub code: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct HttpVerifyRegisterWithPasswordResponse {}

pub async fn register_with_password_finish(
    State(db): State<Arc<Db>>,
    State(sessions_cache): State<Arc<ApiSessionsCache>>,
    Json(request): Json<HttpVerifyRegisterWithPasswordRequest>,
) -> Result<Json<HttpVerifyRegisterWithPasswordResponse>, (StatusCode, String)> {
    // Validate request
    validate_request(&request, &())?;

    // Get session data
    let sessions_key = SessionsCacheKey::RegisterVerification(request.email.clone()).to_string();
    let session_data = match sessions_cache.get(&sessions_key) {
        Some(SessionCache::VerifyRegister(session)) => session,
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

    // Save the user to the database
    let user_id = uuid7().to_string();
    let grafana_user = GrafanaUser {
        user_id: user_id.clone(),
        email: request.email.clone(),
        password_hash: session_data.hashed_password,
        creation_timestamp: get_current_datetime(),
    };
    if let Err(err) = db.add_new_user(&grafana_user).await {
        error!("Failed to create user: {:?}", err);
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            CloudApiErrors::DatabaseError.to_string(),
        ));
    }

    return Ok(Json(HttpVerifyRegisterWithPasswordResponse {}));
}

#[cfg(feature = "cloud_db_tests")]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        http::cloud::register::register_with_password_start::{
            HttpRegisterWithPasswordRequest, HttpRegisterWithPasswordResponse,
        },
        structs::cloud::cloud_http_endpoints::HttpCloudEndpoint,
        test_utils::test_utils::{
            convert_response, convert_response_into_error_string, create_test_app,
        },
    };
    use axum::{
        body::Body,
        extract::ConnectInfo,
        http::{Method, Request},
    };
    use rand::{distributions::Alphanumeric, thread_rng, Rng};
    use std::net::SocketAddr;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_register() {
        let test_app = create_test_app(false).await;

        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();

        let email = format!("{rand_string}@gmail.com");
        let password = format!("Password123");

        // Register user
        let register_payload = HttpRegisterWithPasswordRequest {
            email: email.to_string(),
            password: password.to_string(),
        };

        let ip: ConnectInfo<SocketAddr> = ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 8080)));
        let json = serde_json::to_string(&register_payload).unwrap();

        let req = Request::builder()
            .method(Method::POST)
            .header("content-type", "application/json")
            .uri(format!(
                "/cloud/public{}",
                HttpCloudEndpoint::RegisterWithPasswordStart.to_string()
            ))
            .extension(ip)
            .body(Body::from(json))
            .unwrap();

        // Send request
        let register_response = test_app.clone().oneshot(req).await.unwrap();
        // Validate response
        convert_response::<HttpRegisterWithPasswordResponse>(register_response)
            .await
            .unwrap();

        // Validate register
        let verify_register_payload = HttpVerifyRegisterWithPasswordRequest {
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
                HttpCloudEndpoint::RegisterWithPasswordFinish.to_string()
            ))
            .extension(ip)
            .body(Body::from(json))
            .unwrap();

        // send request to app and get response
        let verify_register_response = test_app.clone().oneshot(req).await.unwrap();
        assert_eq!(verify_register_response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_register_invalid_email() {
        let test_app = create_test_app(false).await;

        let email = format!("@gmail.com");
        let password = format!("Password123");

        // Register user
        let register_payload = HttpRegisterWithPasswordRequest {
            email: email.to_string(),
            password: password.to_string(),
        };

        let ip: ConnectInfo<SocketAddr> = ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 8080)));
        let json = serde_json::to_string(&register_payload).unwrap();

        let req = Request::builder()
            .method(Method::POST)
            .header("content-type", "application/json")
            .uri(format!(
                "/cloud/public{}",
                HttpCloudEndpoint::RegisterWithPasswordStart.to_string()
            ))
            .extension(ip)
            .body(Body::from(json))
            .unwrap();

        // Send request
        let register_response = test_app.clone().oneshot(req).await.unwrap();
        // Validate response, should be an error
        convert_response::<HttpRegisterWithPasswordResponse>(register_response)
            .await
            .unwrap_err();

        // Register with email without domain
        let email = format!("@gmail.com");

        // Register user
        let register_payload = HttpRegisterWithPasswordRequest {
            email: email.to_string(),
            password: password.to_string(),
        };

        let ip: ConnectInfo<SocketAddr> = ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 8080)));
        let json = serde_json::to_string(&register_payload).unwrap();

        let req = Request::builder()
            .method(Method::POST)
            .header("content-type", "application/json")
            .uri(format!(
                "/cloud/public{}",
                HttpCloudEndpoint::RegisterWithPasswordStart.to_string()
            ))
            .extension(ip)
            .body(Body::from(json))
            .unwrap();

        // Send request
        let register_response = test_app.clone().oneshot(req).await.unwrap();
        // Validate response, should be an error
        convert_response::<HttpRegisterWithPasswordResponse>(register_response)
            .await
            .unwrap_err();
    }

    #[tokio::test]
    async fn test_invalid_password() {
        let test_app = create_test_app(false).await;

        let ip: ConnectInfo<SocketAddr> = ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 8080)));
        let uri = format!(
            "/cloud/public{}",
            HttpCloudEndpoint::RegisterWithPasswordStart.to_string()
        );

        {
            let app = test_app.clone();
            let payload = HttpRegisterWithPasswordRequest {
                email: "test@test.com".to_string(),
                password: "dfsdsfa2asdada".to_string(),
            };

            let json = serde_json::to_string(&payload).unwrap();
            let req = Request::builder()
                .method(Method::POST)
                .header("content-type", "application/json")
                .uri(uri.clone())
                .extension(ip)
                .body(Body::from(json))
                .unwrap();

            let res = app.oneshot(req).await.unwrap();
            let status = res.status();
            let message = convert_response_into_error_string(res).await.unwrap();
            let expected_message = "Password do not contain uppercase letter".to_string();
            assert_eq!(status, StatusCode::BAD_REQUEST);
            assert_eq!(message, expected_message);
        }
        {
            let app = test_app.clone();
            let payload = HttpRegisterWithPasswordRequest {
                email: "test@test.com".to_string(),
                password: "dA4ds".to_string(),
            };
            let json = serde_json::to_string(&payload).unwrap();

            let req = Request::builder()
                .method(Method::POST)
                .header("content-type", "application/json")
                .uri(uri.clone())
                .extension(ip)
                .body(Body::from(json))
                .unwrap();
            let res = app.oneshot(req).await.unwrap();
            let status = res.status();
            let message = convert_response_into_error_string(res).await.unwrap();
            let expected_message = "Password is too short".to_string();
            assert_eq!(status, StatusCode::BAD_REQUEST);
            assert_eq!(message, expected_message);
        }
        {
            let app = test_app.clone();
            let payload = HttpRegisterWithPasswordRequest {
                email: "test@test.com".to_string(),
                password: "Ab1aaaaaa¡".to_string(),
            };
            let json = serde_json::to_string(&payload).unwrap();

            let req = Request::builder()
                .method(Method::POST)
                .header("content-type", "application/json")
                .uri(uri.clone())
                .extension(ip)
                .body(Body::from(json))
                .unwrap();
            let res = app.oneshot(req).await.unwrap();
            let status = res.status();
            let message = convert_response_into_error_string(res).await.unwrap();
            let expected_message = "Password contains non-ascii characters".to_string();
            assert_eq!(status, StatusCode::BAD_REQUEST);
            assert_eq!(message, expected_message);
        }
    }
}
