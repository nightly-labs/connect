use super::utils::custom_validate_team_id;
use crate::{
    middlewares::auth_middleware::UserId,
    structs::cloud::{api_cloud_errors::CloudApiErrors, team_user_privilege::TeamUserPrivilege},
};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    Extension, Json,
};
use database::db::Db;
use garde::Validate;
use log::error;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS, Validate)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpGetTeamUsersPrivilegesRequest {
    #[garde(custom(custom_validate_team_id))]
    pub team_id: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpGetTeamUsersPrivilegesResponse {
    pub users_privileges: Vec<TeamUserPrivilege>,
}

pub async fn get_team_users_privileges(
    State(db): State<Arc<Db>>,
    Extension(user_id): Extension<UserId>,
    Query(request): Query<HttpGetTeamUsersPrivilegesRequest>,
) -> Result<Json<HttpGetTeamUsersPrivilegesResponse>, (StatusCode, String)> {
    // Get user data
    match db.get_team_by_team_id(None, &request.team_id).await {
        Ok(Some(team)) => {
            // Check if user is a admin of this team
            if team.team_admin_id != user_id {
                return Err((
                    StatusCode::BAD_REQUEST,
                    CloudApiErrors::InsufficientPermissions.to_string(),
                ));
            }

            if team.deactivated_at != None {
                return Err((
                    StatusCode::BAD_REQUEST,
                    CloudApiErrors::TeamDoesNotExist.to_string(),
                ));
            }

            // Check team type
            if team.personal {
                return Err((
                    StatusCode::BAD_REQUEST,
                    CloudApiErrors::ActionForbiddenForPersonalTeam.to_string(),
                ));
            }

            let team_privileges = match db.get_privileges_by_team_id(&request.team_id).await {
                Ok(privileges) => {
                    if !privileges
                        .iter()
                        .any(|privilege| privilege.user_id == user_id)
                    {
                        return Err((
                            StatusCode::UNAUTHORIZED,
                            CloudApiErrors::InsufficientPermissions.to_string(),
                        ));
                    }

                    privileges
                }
                Err(err) => {
                    error!("Failed to get user privileges: {:?}", err);
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        CloudApiErrors::DatabaseError.to_string(),
                    ));
                }
            };

            // Get team users ids from team_privileges
            let team_members_ids: Vec<String> = team_privileges
                .iter()
                .map(|privilege| privilege.user_id.clone())
                .collect();

            // Get users emails
            let users_ids_emails = match db.get_users_emails_by_ids(&team_members_ids).await {
                Ok(users_ids_emails) => users_ids_emails,
                Err(err) => {
                    error!("Failed to get users ids: {:?}", err);
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        CloudApiErrors::DatabaseError.to_string(),
                    ));
                }
            };

            let final_list = team_privileges
                .iter()
                .map(|privilege| {
                    // Should not happen but just in case
                    let email = match users_ids_emails.get(&privilege.user_id) {
                        Some(email) => email.clone(),
                        None => "".to_string(),
                    };

                    TeamUserPrivilege {
                        app_id: privilege.app_id.clone(),
                        user_email: email.clone(),
                        privilege: privilege.privilege_level.clone(),
                    }
                })
                .collect();

            Ok(Json(HttpGetTeamUsersPrivilegesResponse {
                users_privileges: final_list,
            }))
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
    use crate::{
        env::JWT_SECRET,
        http::cloud::register_new_app::HttpRegisterNewAppRequest,
        structs::cloud::cloud_http_endpoints::HttpCloudEndpoint,
        test_utils::test_utils::{
            add_test_app, add_test_team, add_user_to_test_team, convert_response, create_test_app,
            generate_valid_name, register_and_login_random_user,
        },
    };
    use axum::{
        body::Body,
        extract::ConnectInfo,
        http::{Method, Request},
    };
    use database::structs::privilege_level::PrivilegeLevel;
    use std::net::SocketAddr;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_get_team_metadata() {
        let test_app = create_test_app(false).await;

        let (auth_token, admin_email, _password) = register_and_login_random_user(&test_app).await;

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

        // Register 10 users and invite them to the team
        let mut users_email = Vec::new();
        for _ in 0..10 {
            let (app_user_auth_token, app_user_email, _app_user_password) =
                register_and_login_random_user(&test_app).await;

            // Invite user to the first three teams
            add_user_to_test_team(
                &team_id,
                &app_user_email,
                &auth_token,
                &app_user_auth_token,
                &test_app,
            )
            .await
            .unwrap();

            users_email.push(app_user_email);
        }

        // Get team metadata
        let ip: ConnectInfo<SocketAddr> = ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 8080)));
        let auth = auth_token.encode(JWT_SECRET()).unwrap();

        let req = Request::builder()
            .method(Method::GET)
            .header("content-type", "application/json")
            .header("authorization", format!("Bearer {auth}"))
            .uri(format!(
                "/cloud/private{}?teamId={team_id}",
                HttpCloudEndpoint::GetTeamUserPrivileges.to_string()
            ))
            .extension(ip.clone())
            .body(Body::empty())
            .unwrap();

        // Send request
        let response = test_app.clone().oneshot(req).await.unwrap();
        // Validate response
        let res = convert_response::<HttpGetTeamUsersPrivilegesResponse>(response)
            .await
            .unwrap();

        assert_eq!(res.users_privileges.len(), 11);
        // Check privileges
        for privilege in res.users_privileges {
            if privilege.user_email == admin_email {
                assert_eq!(privilege.privilege, PrivilegeLevel::Admin);
            } else {
                assert_eq!(privilege.privilege, PrivilegeLevel::Read);
            }
        }
    }
}
