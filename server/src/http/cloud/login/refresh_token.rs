use crate::{
    auth::AuthToken, env::{JWT_PUBLIC_KEY, NONCE}, http::cloud::utils::refresh_auth_token,
    structs::cloud::api_cloud_errors::CloudApiErrors,
};
use axum::{extract::ConnectInfo, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpRefreshRequest {
    pub refresh_token: String,
    pub enforce_ip: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpRefreshResponse {
    pub auth_token: String,
}

pub async fn refresh_token(
    ConnectInfo(ip): ConnectInfo<SocketAddr>,
    Json(request): Json<HttpRefreshRequest>,
) -> Result<Json<HttpRefreshResponse>, (StatusCode, String)> {
    let refresh_token = match AuthToken::decode(&request.refresh_token, &JWT_PUBLIC_KEY(), ip) {
        Ok(token) => token,
        Err(err) => {
            println!("Failed to decode refresh token: {:?}", err);
            return Err((
                StatusCode::BAD_REQUEST,
                CloudApiErrors::RefreshTokenFailure.to_string(),
            ));
        }
    };

    if refresh_token.nonce != NONCE() {
        return Err((StatusCode::UNAUTHORIZED, "Expired token".to_string()));
    }

    let ip = match request.enforce_ip {
        true => Some(ip),
        false => None,
    };

    // Refresh token
    let auth_token = refresh_auth_token(refresh_token, ip)?;

    return Ok(Json(HttpRefreshResponse { auth_token }));
}

#[cfg(feature = "cloud_integration_tests")]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        http::cloud::{
            get_user_metadata::HttpUserMetadataResponse,
            login::login_with_password::{HttpLoginRequest, HttpLoginResponse},
            register::{
                register_with_password_finish::HttpRegisterWithPasswordFinishRequest,
                register_with_password_start::{
                    HttpRegisterWithPasswordStartRequest, HttpRegisterWithPasswordStartResponse,
                },
            },
        },
        structs::cloud::cloud_http_endpoints::HttpCloudEndpoint,
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
    async fn test_refresh_token() {
        let test_app = create_test_app(false).await;

        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();

        let email = format!("{rand_string}@gmail.com");
        let password = format!("Password123");

        // Register user
        let register_payload = HttpRegisterWithPasswordStartRequest {
            email: email.to_string(),
            device: "device".to_string(),
            browser: "browser".to_string(),
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
        convert_response::<HttpRegisterWithPasswordStartResponse>(register_response)
            .await
            .unwrap();

        // Validate register
        let verify_register_payload = HttpRegisterWithPasswordFinishRequest {
            email: email.to_string(),
            // Random code for testing
            auth_code: "123456".to_string(),
            new_password: password.to_string(),
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
        let login = convert_response::<HttpLoginResponse>(login_response)
            .await
            .unwrap();

        // Refresh token
        let refresh_payload = HttpRefreshRequest {
            refresh_token: login.refresh_token,
            enforce_ip: false,
        };

        let json = serde_json::to_string(&refresh_payload).unwrap();
        let req = Request::builder()
            .method(Method::POST)
            .header("content-type", "application/json")
            .uri(format!(
                "/cloud/public{}",
                HttpCloudEndpoint::RefreshToken.to_string()
            ))
            .extension(ip)
            .body(Body::from(json))
            .unwrap();

        // Send request
        let refresh_response = test_app.clone().oneshot(req).await.unwrap();
        // Validate response
        let refresh = convert_response::<HttpRefreshResponse>(refresh_response)
            .await
            .unwrap();

        assert_ne!(login.auth_token, refresh.auth_token);

        // Test if new auth token is valid
        // Get user metadata endpoint
        let ip: ConnectInfo<SocketAddr> = ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 8080)));
        let auth = refresh.auth_token;
        let req = Request::builder()
            .method(Method::GET)
            .header("content-type", "application/json")
            .header("authorization", format!("Bearer {auth}"))
            .uri(format!(
                "/cloud/private{}",
                HttpCloudEndpoint::GetUserMetadata.to_string()
            ))
            .extension(ip.clone())
            .body(Body::empty())
            .unwrap();

        // Send request
        let response = test_app.clone().oneshot(req).await.unwrap();
        // Validate response
        let res = convert_response::<HttpUserMetadataResponse>(response)
            .await
            .unwrap();

        assert_eq!(res.email, email);
        assert_eq!(res.password_set, true);
        assert_eq!(res.passkey_ids.len(), 0);
    }
}
