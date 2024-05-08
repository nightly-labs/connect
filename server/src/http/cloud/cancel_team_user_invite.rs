use super::utils::{custom_validate_team_id, validate_request};
use crate::{
    middlewares::auth_middleware::UserId,
    structs::cloud::{api_cloud_errors::CloudApiErrors, team_invite::TeamInvite},
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
pub struct HttpCancelTeamUserInviteRequest {
    #[garde(custom(custom_validate_team_id))]
    pub team_id: String,
    #[garde(email)]
    pub user_email: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct HttpCancelTeamUserInviteResponse {}

pub async fn cancel_team_user_invite(
    State(db): State<Arc<Db>>,
    Extension(user_id): Extension<UserId>,
    Json(request): Json<HttpCancelTeamUserInviteRequest>,
) -> Result<Json<HttpCancelTeamUserInviteResponse>, (StatusCode, String)> {
    // Validate request
    validate_request(&request, &())?;

    // Get team data and perform checks
    match db.get_team_by_team_id(None, &request.team_id).await {
        Ok(Some(team)) => {
            // Check if user is a admin of this team
            if team.team_admin_id != user_id {
                return Err((
                    StatusCode::BAD_REQUEST,
                    CloudApiErrors::InsufficientPermissions.to_string(),
                ));
            }

            // Check team type
            if team.personal {
                return Err((
                    StatusCode::BAD_REQUEST,
                    CloudApiErrors::ActionForbiddenForPersonalTeam.to_string(),
                ));
            }

            // Check if invite exists
            match db.get_invites_by_team_id(&request.team_id, true).await {
                Ok(invites) => {
                    let team_invites: Vec<TeamInvite> = invites
                        .into_iter()
                        .map(|invite| TeamInvite::from(invite))
                        .collect();

                    // Find the invite
                    let invite = team_invites
                        .iter()
                        .find(|invite| invite.user_email == request.user_email);

                    if let None = invite {
                        return Err((
                            StatusCode::BAD_REQUEST,
                            CloudApiErrors::InviteDoesNotExist.to_string(),
                        ));
                    }
                }
                Err(err) => {
                    error!("Failed to get team invites: {:?}", err);
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        CloudApiErrors::DatabaseError.to_string(),
                    ));
                }
            }

            // Cancel invite
            // Cancel the invite
            match db
                .cancel_team_invite(&request.team_id, &request.user_email)
                .await
            {
                Ok(_) => {
                    return Ok(Json(HttpCancelTeamUserInviteResponse {}));
                }
                Err(err) => {
                    error!("Failed to cancel invite: {:?}", err);
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        CloudApiErrors::DatabaseError.to_string(),
                    ));
                }
            }
        }
        Ok(None) => {
            return Err((
                StatusCode::BAD_REQUEST,
                CloudApiErrors::TeamDoesNotExist.to_string(),
            ));
        }
        Err(err) => {
            error!("Failed to get team: {:?}", err);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::DatabaseError.to_string(),
            ));
        }
    }
}

#[cfg(feature = "cloud_integration_tests")]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::test_utils::get_test_team_user_invites;
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
    async fn test_cancel_team_user_invite() {
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

        let _ = add_test_app(&request, &auth_token, &test_app)
            .await
            .unwrap();

        let mut users_emails: Vec<String> = vec![];
        // Invite a few users to the team
        for _ in 0..3 {
            // Register new user
            let (_test_user_auth_token, test_user_email, _test_user_password) =
                register_and_login_random_user(&test_app).await;

            users_emails.push(test_user_email.clone());

            // Create invite
            invite_user_to_test_team(&team_id, &test_user_email, &auth_token, &test_app)
                .await
                .unwrap();
        }

        // Check if team invite exists
        let team_invites = get_test_team_user_invites(&team_id, &auth_token, &test_app)
            .await
            .unwrap();

        assert_eq!(team_invites.team_invites.len(), 3);
        assert_eq!(team_invites.team_invites[0].user_email, users_emails[2]);
        assert_eq!(team_invites.team_invites[1].user_email, users_emails[1]);
        assert_eq!(team_invites.team_invites[2].user_email, users_emails[0]);

        // Cancel team invite for the first user
        let request = HttpCancelTeamUserInviteRequest {
            team_id: team_id.clone(),
            user_email: users_emails[0].clone(),
        };

        let ip: ConnectInfo<SocketAddr> = ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 8080)));
        let json = serde_json::to_string(&request).unwrap();
        let auth = auth_token.encode(JWT_SECRET()).unwrap();

        let req = Request::builder()
            .method(Method::POST)
            .header("content-type", "application/json")
            .header("authorization", format!("Bearer {auth}"))
            .uri(format!(
                "/cloud/private{}",
                HttpCloudEndpoint::CancelTeamUserInvite.to_string()
            ))
            .extension(ip.clone())
            .body(Body::from(json))
            .unwrap();

        // Send request
        let response = test_app.clone().oneshot(req).await.unwrap();
        // Validate response
        convert_response::<HttpCancelTeamUserInviteResponse>(response)
            .await
            .unwrap();

        // Get team invites
        let resp = get_test_team_user_invites(&team_id, &auth_token, &test_app)
            .await
            .unwrap();

        assert_eq!(resp.team_invites.len(), 2);
        assert_eq!(resp.team_invites[0].user_email, users_emails[2]);
        assert_eq!(resp.team_invites[1].user_email, users_emails[1]);

        // Try to cancel the same invite again
        let request = HttpCancelTeamUserInviteRequest {
            team_id: team_id.clone(),
            user_email: users_emails[0].clone(),
        };

        let ip: ConnectInfo<SocketAddr> = ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 8080)));
        let json = serde_json::to_string(&request).unwrap();
        let auth = auth_token.encode(JWT_SECRET()).unwrap();

        let req = Request::builder()
            .method(Method::POST)
            .header("content-type", "application/json")
            .header("authorization", format!("Bearer {auth}"))
            .uri(format!(
                "/cloud/private{}",
                HttpCloudEndpoint::CancelTeamUserInvite.to_string()
            ))
            .extension(ip.clone())
            .body(Body::from(json))
            .unwrap();

        // Send request
        let response = test_app.clone().oneshot(req).await.unwrap();
        // Validate response
        let err = convert_response::<HttpCancelTeamUserInviteResponse>(response)
            .await
            .unwrap_err();

        assert_eq!(
            err.to_string(),
            CloudApiErrors::InviteDoesNotExist.to_string()
        );
    }
}
