use crate::{
    middlewares::auth_middleware::UserId,
    structs::cloud::api_cloud_errors::CloudApiErrors,
    utils::{custom_validate_uuid, validate_request},
};
use axum::{extract::State, http::StatusCode, Extension, Json};
use database::db::Db;
use garde::Validate;
use log::error;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS, Validate)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpCancelUserTeamInviteRequest {
    #[garde(custom(custom_validate_uuid))]
    pub team_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct HttpCancelUserTeamInviteResponse {}

pub async fn cancel_user_team_invite(
    State(db): State<Option<Arc<Db>>>,
    Extension(user_id): Extension<UserId>,
    Json(request): Json<HttpCancelUserTeamInviteRequest>,
) -> Result<Json<HttpCancelUserTeamInviteResponse>, (StatusCode, String)> {
    // Db connection has already been checked in the middleware
    let db = db.as_ref().ok_or((
        StatusCode::INTERNAL_SERVER_ERROR,
        CloudApiErrors::CloudFeatureDisabled.to_string(),
    ))?;

    // Validate request
    validate_request(&request, &())?;

    // Get user data
    let user = match db.get_user_by_user_id(&user_id).await {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Err((
                StatusCode::BAD_REQUEST,
                CloudApiErrors::UserDoesNotExist.to_string(),
            ));
        }
        Err(err) => {
            error!("Failed to get user by user_id: {:?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::DatabaseError.to_string(),
            ));
        }
    };

    // Check if invite exists
    // Get active invites for the user
    match db.get_invites_by_user_email(&user.email, true).await {
        Ok(invites) => {
            // Find the invite
            let invite = invites
                .iter()
                .find(|invite| invite.team_id == request.team_id);

            if let None = invite {
                return Err((
                    StatusCode::BAD_REQUEST,
                    CloudApiErrors::InviteDoesNotExist.to_string(),
                ));
            }
        }
        Err(err) => {
            error!("Failed to get user team invites: {:?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::DatabaseError.to_string(),
            ));
        }
    }

    // Invite exists, cancel it
    match db.cancel_team_invite(&request.team_id, &user.email).await {
        Ok(_) => Ok(Json(HttpCancelUserTeamInviteResponse {})),
        Err(err) => {
            error!("Failed to cancel team invite: {:?}", err);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::DatabaseError.to_string(),
            ))
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
            get_test_user_team_invites, invite_user_to_test_team, register_and_login_random_user,
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
    async fn test_cancel_user_team_invite() {
        let test_app = create_test_app(false).await;

        let (auth_token, _email, _password) = register_and_login_random_user(&test_app).await;

        let num_of_teams = 4;
        let mut team_ids = Vec::new();

        // Create teams
        for _ in 0..num_of_teams {
            let team_name = generate_valid_name();
            let team_id = add_test_team(&team_name, &auth_token, &test_app, false)
                .await
                .unwrap();
            team_ids.push(team_id);
        }

        let mut app_ids = Vec::new();
        // Register 3 + [team index] apps for each team
        for (j, team_id) in team_ids.iter().enumerate() {
            let mut team_app_ids = Vec::new();
            for _ in 0..3 + j {
                let app_name = generate_valid_name();
                let request = HttpRegisterNewAppRequest {
                    team_id: team_id.clone(),
                    app_name: app_name.clone(),
                    whitelisted_domains: vec![],
                    ack_public_keys: vec![],
                };
                let app_id = add_test_app(&request, &auth_token, &test_app)
                    .await
                    .unwrap();
                team_app_ids.push(app_id);
            }

            app_ids.push(team_app_ids);
        }

        // Register new user
        let (app_user_auth_token, app_user_email, _app_user_password) =
            register_and_login_random_user(&test_app).await;

        // Invite user to the first three teams
        for (_, team_id) in team_ids.iter().enumerate().take(3) {
            invite_user_to_test_team(&team_id, &app_user_email, &auth_token, &test_app)
                .await
                .unwrap();
        }

        // Get user team invites
        let user_team_invites = get_test_user_team_invites(&app_user_auth_token, &test_app)
            .await
            .unwrap();
        assert!(user_team_invites.team_invites.len() == 3);

        // Reject team invites to first team
        let request = HttpCancelUserTeamInviteRequest {
            team_id: team_ids[0].clone(),
        };

        let ip: ConnectInfo<SocketAddr> = ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 8080)));
        let json = serde_json::to_string(&request).unwrap();
        let auth = app_user_auth_token.encode(JWT_SECRET()).unwrap();

        let req = Request::builder()
            .method(Method::POST)
            .header("content-type", "application/json")
            .header("authorization", format!("Bearer {auth}"))
            .uri(format!(
                "/cloud/private{}",
                HttpCloudEndpoint::CancelUserTeamInvite.to_string()
            ))
            .extension(ip)
            .body(Body::from(json))
            .unwrap();

        // Send request
        let response = test_app.clone().oneshot(req).await.unwrap();
        // Validate response
        let _response = convert_response::<super::HttpCancelUserTeamInviteResponse>(response)
            .await
            .unwrap();

        // Get user team invites
        let user_team_invites = get_test_user_team_invites(&app_user_auth_token, &test_app)
            .await
            .unwrap();
        assert!(user_team_invites.team_invites.len() == 2);

        // Try to cancel invite that does not exist
        let request = HttpCancelUserTeamInviteRequest {
            team_id: team_ids[0].clone(),
        };

        let ip: ConnectInfo<SocketAddr> = ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 8080)));
        let json = serde_json::to_string(&request).unwrap();
        let auth = app_user_auth_token.encode(JWT_SECRET()).unwrap();

        let req = Request::builder()
            .method(Method::POST)
            .header("content-type", "application/json")
            .header("authorization", format!("Bearer {auth}"))
            .uri(format!(
                "/cloud/private{}",
                HttpCloudEndpoint::CancelUserTeamInvite.to_string()
            ))
            .extension(ip)
            .body(Body::from(json))
            .unwrap();

        // Send request
        let response = test_app.clone().oneshot(req).await.unwrap();
        // Validate response
        let response = convert_response::<HttpCancelUserTeamInviteResponse>(response)
            .await
            .unwrap_err();
        assert_eq!(
            response.to_string(),
            CloudApiErrors::InviteDoesNotExist.to_string()
        );
    }
}
