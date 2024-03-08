use crate::{
    auth::AuthToken,
    env::{JWT_SECRET, NONCE},
    structs::api_cloud_errors::CloudApiErrors,
    utils::validate_request,
};
use axum::{
    extract::{ConnectInfo, State},
    http::StatusCode,
    Json,
};
use database::db::Db;
use garde::Validate;
use log::error;
use pwhash::bcrypt;
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS, Validate)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpLoginRequest {
    #[garde(email)]
    pub email: String,
    #[garde(ascii, length(min = 6, max = 30))]
    pub password: String,
    #[garde(skip)]
    pub enforce_ip: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct HttpLoginResponse {
    pub user_id: String,
    pub auth_token: String,
    pub refresh_token: String,
}

pub async fn login_with_password(
    ConnectInfo(ip): ConnectInfo<SocketAddr>,
    State(db): State<Option<Arc<Db>>>,
    Json(request): Json<HttpLoginRequest>,
) -> Result<Json<HttpLoginResponse>, (StatusCode, String)> {
    // Db connection has already been checked in the middleware
    let db = db.as_ref().ok_or((
        StatusCode::INTERNAL_SERVER_ERROR,
        CloudApiErrors::CloudFeatureDisabled.to_string(),
    ))?;

    // Validate request
    validate_request(&request, &())?;

    // Check if user exists
    let user = match db.get_user_by_email(&request.email).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Err((
                StatusCode::BAD_REQUEST,
                CloudApiErrors::UserDoesNotExist.to_string(),
            ));
        }
        Err(err) => {
            error!("Failed to get user by email: {:?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::DatabaseError.to_string(),
            ));
        }
    };

    // Verify password
    if bcrypt::verify(
        format!("{}_{}", NONCE(), request.password),
        &user.password_hash,
    ) == false
    {
        return Err((
            StatusCode::BAD_REQUEST,
            CloudApiErrors::IncorrectPassword.to_string(),
        ));
    }

    // Generate tokens
    let ip = if request.enforce_ip { Some(ip) } else { None };
    // Access token
    let token = match AuthToken::new_access(&user.user_id, ip).encode(JWT_SECRET()) {
        Ok(token) => token,
        Err(err) => {
            error!("Failed to create access token: {:?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::AccessTokenFailure.to_string(),
            ));
        }
    };
    // Refresh token
    let refresh_token = match AuthToken::new_refresh(&user.user_id, ip).encode(JWT_SECRET()) {
        Ok(token) => token,
        Err(err) => {
            error!("Failed to create refresh token: {:?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::RefreshTokenFailure.to_string(),
            ));
        }
    };

    return Ok(Json(HttpLoginResponse {
        auth_token: token,
        refresh_token,
        user_id: user.user_id,
    }));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        http::cloud::register_with_password::{
            HttpRegisterWithPasswordRequest, HttpRegisterWithPasswordResponse,
        },
        structs::cloud_http_endpoints::HttpCloudEndpoint,
        test_utils::test_utils::{convert_response, create_test_app},
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
    async fn test_login() {
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

        // Login user
        let login_payload = HttpLoginRequest {
            email: email.to_string(),
            password: password.to_string(),
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

    #[tokio::test]
    async fn test_login_as_non_existing_user() {
        let test_app = create_test_app(false).await;

        let email = format!("trylogin@gmail.com");
        let password = format!("Password123");

        // Login user, should fail
        let login_payload = HttpLoginRequest {
            email: email.to_string(),
            password: password.to_string(),
            enforce_ip: false,
        };

        let ip: ConnectInfo<SocketAddr> = ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 8080)));
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
        let err = convert_response::<HttpLoginResponse>(login_response)
            .await
            .unwrap_err();

        assert_eq!(
            err.to_string(),
            CloudApiErrors::UserDoesNotExist.to_string()
        );
    }

    #[tokio::test]
    async fn test_login_incorrect_password() {
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

        // Login user with wring password, should fail
        let login_payload = HttpLoginRequest {
            email: email.to_string(),
            password: "WrongPassword".to_string(),
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
        let err = convert_response::<HttpLoginResponse>(login_response)
            .await
            .unwrap_err();

        assert_eq!(
            err.to_string(),
            CloudApiErrors::IncorrectPassword.to_string()
        );
    }
}
