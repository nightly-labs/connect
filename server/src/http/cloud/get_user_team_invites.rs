use crate::{
    middlewares::auth_middleware::UserId,
    structs::cloud::{api_cloud_errors::CloudApiErrors, team_invite::TeamInvite},
};
use axum::{extract::State, http::StatusCode, Extension, Json};
use database::db::Db;
use log::error;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpGetUserTeamInvitesResponse {
    pub team_invites: Vec<TeamInvite>,
}

pub async fn get_user_team_invites(
    State(db): State<Arc<Db>>,
    Extension(user_id): Extension<UserId>,
) -> Result<Json<HttpGetUserTeamInvitesResponse>, (StatusCode, String)> {
    // Get user data and perform checks
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

    // Get active invites for the team
    match db.get_invites_by_user_email(&user.email, true).await {
        Ok(invites) => {
            let team_invites: Vec<TeamInvite> = invites
                .into_iter()
                .map(|invite| TeamInvite::from(invite))
                .collect();

            Ok(Json(HttpGetUserTeamInvitesResponse { team_invites }))
        }
        Err(err) => {
            error!("Failed to get user team invites: {:?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::DatabaseError.to_string(),
            ));
        }
    }
}

#[cfg(feature = "cloud_db_tests")]
#[cfg(test)]
mod tests {
    use crate::auth::AuthToken;
    use crate::http::cloud::get_user_team_invites::HttpGetUserTeamInvitesResponse;
    use crate::test_utils::test_utils::get_test_user_team_invites;
    use crate::{
        env::JWT_SECRET,
        http::cloud::register_new_app::HttpRegisterNewAppRequest,
        structs::cloud::cloud_http_endpoints::HttpCloudEndpoint,
        test_utils::test_utils::{
            add_test_app, add_test_team, convert_response, create_test_app, generate_valid_name,
            invite_user_to_test_team, register_and_login_random_user,
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
    async fn test_user_team_invites() {
        let test_app = create_test_app(false).await;

        // Register 3 teams
        let mut team_ids: Vec<String> = vec![];
        let mut admin_auth_tokens: Vec<AuthToken> = vec![];

        for _ in 0..3 {
            let (auth_token, _email, _password) = register_and_login_random_user(&test_app).await;

            admin_auth_tokens.push(auth_token.clone());

            // Register new team
            let team_name = generate_valid_name();
            let team_id = add_test_team(&team_name, &auth_token, &test_app, false)
                .await
                .unwrap();

            team_ids.push(team_id.clone());

            // Register app under the team
            let app_name = generate_valid_name();
            let request = HttpRegisterNewAppRequest {
                team_id: team_id.clone(),
                app_name: app_name.clone(),
            };

            let _ = add_test_app(&request, &auth_token, &test_app)
                .await
                .unwrap();
        }

        // Register new user
        let (test_user_auth_token, test_user_email, _test_user_password) =
            register_and_login_random_user(&test_app).await;

        // Create invites for first two teams
        invite_user_to_test_team(
            &team_ids[0],
            &test_user_email,
            &admin_auth_tokens[0],
            &test_app,
        )
        .await
        .unwrap();

        invite_user_to_test_team(
            &team_ids[1],
            &test_user_email,
            &admin_auth_tokens[1],
            &test_app,
        )
        .await
        .unwrap();

        // Get user team invites
        let ip: ConnectInfo<SocketAddr> = ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 8080)));
        let auth = test_user_auth_token.encode(JWT_SECRET()).unwrap();

        let req = Request::builder()
            .method(Method::GET)
            .header("content-type", "application/json")
            .header("authorization", format!("Bearer {auth}"))
            .uri(format!(
                "/cloud/private{}",
                HttpCloudEndpoint::GetUserTeamInvites.to_string()
            ))
            .extension(ip.clone())
            .body(Body::empty())
            .unwrap();

        // Send request
        let response = test_app.clone().oneshot(req).await.unwrap();
        // Validate response
        let res = convert_response::<HttpGetUserTeamInvitesResponse>(response)
            .await
            .unwrap();

        assert_eq!(res.team_invites.len(), 2);

        // Invite user to the last team
        invite_user_to_test_team(
            &team_ids[2],
            &test_user_email,
            &admin_auth_tokens[2],
            &test_app,
        )
        .await
        .unwrap();

        // Get user team invites
        let invites = get_test_user_team_invites(&test_user_auth_token, &test_app)
            .await
            .unwrap();

        assert_eq!(invites.team_invites.len(), 3);
    }

    #[tokio::test]
    async fn test_no_invites() {
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
        };

        // unwrap err as it should have failed
        let _ = add_test_app(&request, &auth_token, &test_app)
            .await
            .unwrap();

        // Get team invites
        let resp = get_test_user_team_invites(&auth_token, &test_app)
            .await
            .unwrap();

        assert_eq!(resp.team_invites.len(), 0);
    }
}
