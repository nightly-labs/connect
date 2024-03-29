use crate::{
    env::NONCE,
    structs::cloud::api_cloud_errors::CloudApiErrors,
    utils::{custom_validate_new_password, validate_request},
};
use axum::{extract::State, http::StatusCode, Json};
use database::{
    db::Db,
    tables::{grafana_users::table_struct::GrafanaUser, utils::get_current_datetime},
};
use garde::Validate;
use log::error;
use pwhash::bcrypt;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ts_rs::TS;
use uuid7::uuid7;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS, Validate)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpRegisterWithPasswordRequest {
    #[garde(email)]
    pub email: String,
    #[garde(custom(custom_validate_new_password))]
    pub password: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpRegisterWithPasswordResponse {
    pub user_id: String,
}

pub async fn register_with_password(
    State(db): State<Option<Arc<Db>>>,
    Json(request): Json<HttpRegisterWithPasswordRequest>,
) -> Result<Json<HttpRegisterWithPasswordResponse>, (StatusCode, String)> {
    // Db connection has already been checked in the middleware
    let db = db.as_ref().ok_or((
        StatusCode::INTERNAL_SERVER_ERROR,
        CloudApiErrors::CloudFeatureDisabled.to_string(),
    ))?;

    // Validate request
    validate_request(&request, &())?;

    // Check if user already exists
    match db.get_user_by_email(&request.email).await {
        Ok(Some(_)) => {
            return Err((
                StatusCode::BAD_REQUEST,
                CloudApiErrors::EmailAlreadyExists.to_string(),
            ))
        }
        Ok(None) => {
            // Continue
        }
        Err(err) => {
            error!("Failed to check if user exists: {:?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::DatabaseError.to_string(),
            ));
        }
    }

    let hashed_password = bcrypt::hash(format!("{}_{}", NONCE(), request.password.clone()))
        .map_err(|e| {
            error!("Failed to hash password: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::InternalServerError.to_string(),
            )
        })?;

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
            CloudApiErrors::DatabaseError.to_string(),
        ));
    }

    return Ok(Json(HttpRegisterWithPasswordResponse { user_id }));
}

#[cfg(feature = "cloud_db_tests")]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
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
                HttpCloudEndpoint::RegisterWithPassword.to_string()
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

        // Try to register the same user again, should fail
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
                HttpCloudEndpoint::RegisterWithPassword.to_string()
            ))
            .extension(ip)
            .body(Body::from(json))
            .unwrap();

        // Send request
        let register_response = test_app.clone().oneshot(req).await.unwrap();
        // Validate response
        convert_response::<HttpRegisterWithPasswordResponse>(register_response)
            .await
            .unwrap_err();
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
                HttpCloudEndpoint::RegisterWithPassword.to_string()
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
                HttpCloudEndpoint::RegisterWithPassword.to_string()
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
            HttpCloudEndpoint::RegisterWithPassword.to_string()
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
