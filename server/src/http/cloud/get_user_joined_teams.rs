use crate::{
    middlewares::auth_middleware::UserId,
    state::AppId,
    structs::cloud::{
        api_cloud_errors::CloudApiErrors,
        app_info::AppInfo,
        joined_team::{JoinedTeam, TeamId},
        user_privilege::UserPrivilege,
    },
};
use axum::{extract::State, http::StatusCode, Extension, Json};
use database::db::Db;
use log::error;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct HttpGetUserJoinedTeamsResponse {
    pub teams: HashMap<TeamId, JoinedTeam>,
    pub teams_apps: HashMap<TeamId, Vec<AppInfo>>,
    pub user_privileges: HashMap<TeamId, HashMap<AppId, UserPrivilege>>,
}

pub async fn get_user_joined_teams(
    State(db): State<Option<Arc<Db>>>,
    Extension(user_id): Extension<UserId>,
) -> Result<Json<HttpGetUserJoinedTeamsResponse>, (StatusCode, String)> {
    // Db connection has already been checked in the middleware
    let db = db.as_ref().ok_or((
        StatusCode::INTERNAL_SERVER_ERROR,
        CloudApiErrors::CloudFeatureDisabled.to_string(),
    ))?;

    // Check if user already belongs to the team
    match db.get_joined_teams_by_user_id(&user_id).await {
        Ok(joined_teams) => {
            let mut teams = HashMap::new();
            let mut teams_apps = HashMap::new();
            let mut user_privileges = HashMap::new();

            for (team, admin_email, joined_timestamp, registered_apps) in joined_teams {
                let team_id = team.team_id.clone();

                // Parse joined team
                let joined_team = JoinedTeam {
                    team_id: team.team_id.clone(),
                    team_name: team.team_name,
                    created_at: team.registration_timestamp,
                    creator_email: admin_email,
                    personal: team.personal,
                    joined_at: joined_timestamp,
                };
                teams.insert(team_id.clone(), joined_team);

                // Parse teams apps and user privileges
                let mut apps_info = Vec::new();
                let mut privileges = HashMap::new();

                for (app, privilege) in registered_apps {
                    let app_info: AppInfo = app.into();
                    let privilege: UserPrivilege = privilege.into();

                    privileges.insert(app_info.app_id.clone(), privilege);
                    apps_info.push(app_info);
                }

                if !apps_info.is_empty() {
                    teams_apps.insert(team_id.clone(), apps_info);
                }

                if !privileges.is_empty() {
                    user_privileges.insert(team_id.clone(), privileges);
                }
            }

            Ok(Json(HttpGetUserJoinedTeamsResponse {
                teams,
                teams_apps,
                user_privileges,
            }))
        }
        Err(err) => {
            error!(
                "Failed to get teams and apps membership by user id: {:?}",
                err
            );
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
    use crate::structs::cloud::cloud_http_endpoints::HttpCloudEndpoint;
    use crate::test_utils::test_utils::{
        add_user_to_test_team, generate_valid_name, get_test_user_joined_teams,
    };
    use crate::{
        env::JWT_SECRET,
        http::cloud::register_new_app::HttpRegisterNewAppRequest,
        test_utils::test_utils::{
            add_test_app, add_test_team, convert_response, create_test_app,
            register_and_login_random_user,
        },
    };
    use axum::{
        body::Body,
        extract::ConnectInfo,
        http::{Method, Request},
    };
    use database::tables::utils::get_current_datetime;
    use std::net::SocketAddr;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_get_user_joined_teams() {
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

        // Add user to first team
        let before_first_join = get_current_datetime();
        add_user_to_test_team(
            &team_ids[0],
            &app_user_email,
            &auth_token,
            &app_user_auth_token,
            &test_app,
        )
        .await
        .unwrap();
        let after_first_join = get_current_datetime();

        // Wait for 1 second
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

        // Add user to second team
        add_user_to_test_team(
            &team_ids[1],
            &app_user_email,
            &auth_token,
            &app_user_auth_token,
            &test_app,
        )
        .await
        .unwrap();
        let after_second_join = get_current_datetime();

        // Get user joined teams
        let ip: ConnectInfo<SocketAddr> = ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 8080)));
        let auth = app_user_auth_token.encode(JWT_SECRET()).unwrap();

        let req = Request::builder()
            .method(Method::GET)
            .header("content-type", "application/json")
            .header("authorization", format!("Bearer {auth}"))
            .uri(format!(
                "/cloud/private{}",
                HttpCloudEndpoint::GetUserJoinedTeams.to_string()
            ))
            .extension(ip)
            .body(Body::empty())
            .unwrap();

        // Send request
        let response = test_app.clone().oneshot(req).await.unwrap();
        // Validate response
        let response = convert_response::<super::HttpGetUserJoinedTeamsResponse>(response)
            .await
            .unwrap();

        // Check if dates for team join are correct
        assert!(response.teams.get(&team_ids[0]).unwrap().joined_at >= before_first_join);
        assert!(response.teams.get(&team_ids[0]).unwrap().joined_at <= after_first_join);
        assert!(response.teams.get(&team_ids[1]).unwrap().joined_at >= after_first_join);
        assert!(response.teams.get(&team_ids[1]).unwrap().joined_at <= after_second_join);

        // Check returned data
        assert!(response.teams.len() == 2);
        assert!(response.teams_apps.len() == 2);
        assert!(response.teams_apps.get(&team_ids[0]).unwrap().len() == 3);
        assert!(response.teams_apps.get(&team_ids[1]).unwrap().len() == 4);
        assert!(response.user_privileges.len() == 2);
        assert!(response.user_privileges.get(&team_ids[0]).unwrap().len() == 3);
        assert!(response.user_privileges.get(&team_ids[1]).unwrap().len() == 4);

        // Create personal team as a test user
        let personal_team_name = generate_valid_name();
        let personal_team_id =
            add_test_team(&personal_team_name, &app_user_auth_token, &test_app, true)
                .await
                .unwrap();

        // Get user joined teams
        let ip: ConnectInfo<SocketAddr> = ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 8080)));
        let auth = app_user_auth_token.encode(JWT_SECRET()).unwrap();

        let req = Request::builder()
            .method(Method::GET)
            .header("content-type", "application/json")
            .header("authorization", format!("Bearer {auth}"))
            .uri(format!(
                "/cloud/private{}",
                HttpCloudEndpoint::GetUserJoinedTeams.to_string()
            ))
            .extension(ip)
            .body(Body::empty())
            .unwrap();

        // Send request
        let response = test_app.clone().oneshot(req).await.unwrap();
        // Validate response
        let response = convert_response::<super::HttpGetUserJoinedTeamsResponse>(response)
            .await
            .unwrap();

        // Check returned data
        assert!(response.teams.len() == 3);
        assert!(response.teams.get(&personal_team_id).unwrap().personal);
        assert!(response.teams_apps.len() == 2);
        assert!(response.teams_apps.get(&team_ids[0]).unwrap().len() == 3);
        assert!(response.teams_apps.get(&team_ids[1]).unwrap().len() == 4);
        assert!(response.user_privileges.len() == 2);
        assert!(response.user_privileges.get(&team_ids[0]).unwrap().len() == 3);
        assert!(response.user_privileges.get(&team_ids[1]).unwrap().len() == 4);
    }

    #[tokio::test]
    async fn test_get_user_joined_teams_empty_teams() {
        let test_app = create_test_app(false).await;

        let (auth_token, _email, _password) = register_and_login_random_user(&test_app).await;

        // Create personal team
        let personal_team_name = generate_valid_name();
        let personal_team_id = add_test_team(&personal_team_name, &auth_token, &test_app, true)
            .await
            .unwrap();

        // Get user joined teams
        let response = get_test_user_joined_teams(&auth_token, &test_app)
            .await
            .unwrap();

        assert!(response.teams.len() == 1);
        assert!(response.teams.get(&personal_team_id).unwrap().personal);
        assert!(response.teams_apps.len() == 0);
        assert!(response.user_privileges.len() == 0);

        // Add new "normal" team
        let team_name = generate_valid_name();
        let team_id = add_test_team(&team_name, &auth_token, &test_app, false)
            .await
            .unwrap();

        // Get user joined teams
        let response = get_test_user_joined_teams(&auth_token, &test_app)
            .await
            .unwrap();

        assert!(response.teams.len() == 2);
        assert!(response.teams.get(&team_id).unwrap().team_name == team_name);
        assert!(response.teams.get(&team_id).unwrap().personal == false);
        assert!(response.teams_apps.len() == 0);
        assert!(response.user_privileges.len() == 0);
    }

    #[tokio::test]
    async fn test_get_user_joined_teams_empty_account() {
        let test_app = create_test_app(false).await;

        let (auth_token, _email, _password) = register_and_login_random_user(&test_app).await;

        // Get user joined teams
        let response = get_test_user_joined_teams(&auth_token, &test_app)
            .await
            .unwrap();

        assert!(response.teams.len() == 0);
        assert!(response.teams_apps.len() == 0);
        assert!(response.user_privileges.len() == 0);
    }
}
