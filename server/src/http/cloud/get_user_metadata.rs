use crate::{
    middlewares::auth_middleware::UserId, structs::cloud::api_cloud_errors::CloudApiErrors,
};
use axum::Extension;
use axum::{extract::State, http::StatusCode, Json};
use database::db::Db;
use log::error;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpUserMetadataResponse {
    pub user_id: String,
    pub email: String,
    pub password_set: bool,
    pub passkey_ids: Vec<String>,
}

pub async fn get_user_metadata(
    State(db): State<Arc<Db>>,
    Extension(user_id): Extension<UserId>,
) -> Result<Json<HttpUserMetadataResponse>, (StatusCode, String)> {
    // Get user data
    let user_data = match db.get_user_by_user_id(&user_id).await {
        Ok(Some(user_data)) => user_data,
        Ok(None) => {
            return Err((
                StatusCode::BAD_REQUEST,
                CloudApiErrors::UserDoesNotExist.to_string(),
            ))
        }
        Err(err) => {
            error!(
                "Failed to check if user exists: {:?}, user_id: {}",
                err, user_id
            );
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::DatabaseError.to_string(),
            ));
        }
    };

    let response = HttpUserMetadataResponse {
        user_id: user_data.user_id,
        email: user_data.email,
        password_set: user_data.password_hash.is_some(),
        passkey_ids: user_data
            .passkeys
            .unwrap_or_default()
            .iter()
            .map(|p| p.cred_id().to_string())
            .collect(),
    };

    Ok(Json(response))
}

#[cfg(feature = "cloud_db_tests")]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        env::JWT_SECRET,
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
    async fn test_get_user_metadata() {
        let test_app = create_test_app(false).await;

        // Register and login user
        let (auth_token, email, _password) = register_and_login_random_user(&test_app).await;

        // Get user metadata
        let ip: ConnectInfo<SocketAddr> = ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 8080)));
        let auth = auth_token.encode(JWT_SECRET()).unwrap();

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
        assert_eq!(res.user_id, auth_token.user_id);
    }
}
