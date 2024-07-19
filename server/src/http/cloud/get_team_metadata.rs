use super::utils::custom_validate_team_id;
use crate::{
    middlewares::auth_middleware::UserId,
    structs::cloud::{
        api_cloud_errors::CloudApiErrors, app_info::AppInfo, team_metadata::TeamMetadata,
    },
};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    Extension, Json,
};
use database::{
    db::Db, structs::privilege_level::PrivilegeLevel,
    tables::user_app_privileges::table_struct::UserAppPrivilege,
};
use garde::Validate;
use log::error;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS, Validate)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpGetTeamMetadataRequest {
    #[garde(custom(custom_validate_team_id))]
    pub team_id: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpGetTeamMetadataResponse {
    pub team_metadata: TeamMetadata,
    pub team_apps: Vec<AppInfo>,
    pub team_members: Vec<String>,
}

pub async fn get_team_metadata(
    State(db): State<Arc<Db>>,
    Extension(user_id): Extension<UserId>,
    Query(request): Query<HttpGetTeamMetadataRequest>,
) -> Result<Json<HttpGetTeamMetadataResponse>, (StatusCode, String)> {
    // Get user data
    match db.get_team_by_team_id(None, &request.team_id).await {
        Ok(Some(team)) => {
            // Check if user has privileges to access this team
            let mut team_privileges = match db.get_privileges_by_team_id(&request.team_id).await {
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

            if team_privileges.is_empty() && team.team_admin_id == user_id {
                team_privileges.push(UserAppPrivilege {
                    user_id: user_id.clone(),
                    app_id: "".to_string(),
                    privilege_level: PrivilegeLevel::Admin,
                    creation_timestamp: team.registration_timestamp,
                })
            }
            // Get team admin email
            let admin_email = match db.get_user_by_user_id(&team.team_admin_id).await {
                Ok(Some(user)) => user.email,
                Ok(None) => {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        CloudApiErrors::DatabaseError.to_string(),
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

            // Get team data
            let team_metadata = TeamMetadata {
                creator_email: admin_email,
                team_id: team.team_id.clone(),
                team_name: team.team_name,
                personal_team: team.personal,
                created_at: team.registration_timestamp,
            };

            // Get team apps
            let mut registered_apps: Vec<AppInfo> =
                match db.get_registered_apps_by_team_id(&team.team_id).await {
                    Ok(apps) => apps.into_iter().map(|app| app.into()).collect(),
                    Err(err) => {
                        error!("Failed to get registered apps by team_id: {:?}", err);
                        return Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            CloudApiErrors::DatabaseError.to_string(),
                        ));
                    }
                };

            // Get pending domain verifications
            let app_ids = registered_apps
                .iter()
                .map(|app| app.app_id.clone())
                .collect::<Vec<String>>();

            if let Ok(mut pending_domain_verifications) = db
                .get_pending_domain_verifications_by_app_ids(&app_ids)
                .await
            {
                for app in registered_apps.iter_mut() {
                    if let Some(pending_domains) = pending_domain_verifications.get_mut(&app.app_id)
                    {
                        app.whitelisted_domains.append(pending_domains);
                    }
                }
            }

            // Get team users from team_privileges
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

            let team_members: Vec<String> = users_ids_emails.values().cloned().collect();

            // Return data to user
            return Ok(Json(HttpGetTeamMetadataResponse {
                team_metadata,
                team_apps: registered_apps,
                team_members,
            }));
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
    use std::net::SocketAddr;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_get_team_metadata() {
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
                HttpCloudEndpoint::GetTeamMetadata.to_string()
            ))
            .extension(ip.clone())
            .body(Body::empty())
            .unwrap();

        // Send request
        let response = test_app.clone().oneshot(req).await.unwrap();
        // Validate response
        let res = convert_response::<HttpGetTeamMetadataResponse>(response)
            .await
            .unwrap();

        assert_eq!(res.team_metadata.team_id, team_id);
        assert_eq!(res.team_metadata.team_name, team_name);
        assert_eq!(res.team_metadata.personal_team, false);
        assert_eq!(res.team_members.len(), 11);
        assert_eq!(res.team_apps.len(), 1);

        // Check if all users are in the team
        for email in users_email {
            assert!(res.team_members.contains(&email));
        }
    }
}
