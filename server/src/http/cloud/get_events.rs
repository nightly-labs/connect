use crate::{
    middlewares::auth_middleware::UserId,
    structs::cloud::{api_cloud_errors::CloudApiErrors, app_event::AppEvent},
};
use axum::extract::Query;
use axum::Extension;
use axum::{extract::State, http::StatusCode, Json};
use database::db::Db;
use database::structs::pagination_cursor::PaginationCursor;
use garde::Validate;
use log::error;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ts_rs::TS;

use super::utils::{custom_validate_optional_pagination_cursor, custom_validate_uuid};

#[derive(Debug, Clone, Serialize, Deserialize, TS, Validate)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpGetAppEventsRequest {
    #[garde(custom(custom_validate_uuid))]
    pub app_id: String,
    #[garde(custom(custom_validate_optional_pagination_cursor))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_cursor: Option<PaginationCursor>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpGetAppEventsResponse {
    pub events: Vec<AppEvent>,
    pub cursor: Option<PaginationCursor>,
}

pub async fn get_events(
    State(db): State<Arc<Db>>,
    Extension(user_id): Extension<UserId>,
    Query(request): Query<HttpGetAppEventsRequest>,
) -> Result<Json<HttpGetAppEventsResponse>, (StatusCode, String)> {
    // Check if user has sufficient permissions
    match db
        .get_privilege_by_user_id_and_app_id(&user_id, &request.app_id)
        .await
    {
        Ok(Some(_)) => {
            // Get events
            let (events, cursor) = match db
                .get_events_by_app_id(request.pagination_cursor, &request.app_id)
                .await
            {
                Ok((events, cursor)) => {
                    let events = events.into_iter().map(|e| e.into()).collect();
                    (events, cursor)
                }
                Err(e) => {
                    error!("Failed to get events by app id: {:?}", e);
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        CloudApiErrors::InternalServerError.to_string(),
                    ));
                }
            };

            return Ok(Json(HttpGetAppEventsResponse { events, cursor }));
        }
        Ok(None) => {
            return Err((
                StatusCode::FORBIDDEN,
                CloudApiErrors::InsufficientPermissions.to_string(),
            ));
        }
        Err(e) => {
            error!("Failed to get privilege by user id and app id: {:?}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::InternalServerError.to_string(),
            ));
        }
    }
}

#[cfg(feature = "cloud_db_tests")]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        env::JWT_SECRET,
        http::cloud::register_new_app::HttpRegisterNewAppRequest,
        structs::cloud::cloud_http_endpoints::HttpCloudEndpoint,
        test_utils::test_utils::{
            add_test_app, add_test_team, convert_response, create_test_app, generate_valid_name,
            register_and_login_random_user,
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
    async fn test_get_app_events() {
        let test_app = create_test_app(false).await;

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
            whitelisted_domains: vec![],
            ack_public_keys: vec![],
        };

        // unwrap err as it should have failed
        let app_id = add_test_app(&request, &auth_token, &test_app)
            .await
            .unwrap();

        // Get team invites
        let ip: ConnectInfo<SocketAddr> = ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 8080)));
        let auth = auth_token.encode(JWT_SECRET()).unwrap();

        let req = Request::builder()
            .method(Method::GET)
            .header("content-type", "application/json")
            .header("authorization", format!("Bearer {auth}"))
            .uri(format!(
                "/cloud/private{}?appId={app_id}",
                HttpCloudEndpoint::GetEvents.to_string()
            ))
            .extension(ip.clone())
            .body(Body::empty())
            .unwrap();

        // Send request
        let response = test_app.clone().oneshot(req).await.unwrap();
        // Validate response
        let res = convert_response::<HttpGetAppEventsResponse>(response)
            .await
            .unwrap();

        assert_eq!(res.events.len(), 0);
        assert!(res.cursor.is_none());
    }

    #[tokio::test]
    async fn test_get_app_events_forbidden() {
        let test_app = create_test_app(false).await;

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
            whitelisted_domains: vec![],
            ack_public_keys: vec![],
        };

        // unwrap err as it should have failed
        let app_id = add_test_app(&request, &auth_token, &test_app)
            .await
            .unwrap();

        // Register new user
        let (test_user_auth_token, _test_user_email, _test_user_password) =
            register_and_login_random_user(&test_app).await;

        // Get team invites using new user token, should fail
        let ip: ConnectInfo<SocketAddr> = ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 8080)));
        let auth = test_user_auth_token.encode(JWT_SECRET()).unwrap();

        let req = Request::builder()
            .method(Method::GET)
            .header("content-type", "application/json")
            .header("authorization", format!("Bearer {auth}"))
            .uri(format!(
                "/cloud/private{}?appId={app_id}",
                HttpCloudEndpoint::GetEvents.to_string()
            ))
            .extension(ip.clone())
            .body(Body::empty())
            .unwrap();

        // Send request
        let response = test_app.clone().oneshot(req).await.unwrap();
        // Validate response
        let res = convert_response::<HttpGetAppEventsResponse>(response)
            .await
            .unwrap_err();

        assert_eq!(res.to_string(), StatusCode::FORBIDDEN.to_string());
    }
}
