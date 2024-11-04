use super::utils::{custom_validate_uuid, validate_request};
use crate::{
    middlewares::auth_middleware::UserId,
    structs::cloud::{
        api_cloud_errors::CloudApiErrors, new_user_privilege_level::NewUserPrivilegeLevel,
    },
};
use axum::{extract::State, http::StatusCode, Extension, Json};
use database::{
    db::Db,
    structs::privilege_level::PrivilegeLevel,
    tables::{user_app_privileges::table_struct::UserAppPrivilege, utils::get_current_datetime},
};
use garde::Validate;
use log::error;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS, Validate)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct PrivilegeChange {
    #[garde(custom(custom_validate_uuid))]
    pub app_id: String,
    #[garde(email)]
    pub user_email: String,
    #[garde(skip)]
    pub new_privilege_level: NewUserPrivilegeLevel,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS, Validate)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpChangeUsersPrivilegesRequest {
    #[garde(custom(custom_validate_uuid))]
    pub team_id: String,
    #[garde(dive)]
    pub privileges_changes: Vec<PrivilegeChange>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct HttpChangeUsersPrivilegesResponse {}

pub async fn change_user_privileges(
    State(db): State<Arc<Db>>,
    Extension(user_id): Extension<UserId>,
    Json(request): Json<HttpChangeUsersPrivilegesRequest>,
) -> Result<Json<HttpChangeUsersPrivilegesResponse>, (StatusCode, String)> {
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

            // Check if changes can be applied
            let emails: Vec<String> = request
                .privileges_changes
                .iter()
                .map(|x| &x.user_email)
                .collect::<HashSet<&String>>()
                .into_iter()
                .map(|x| x.clone())
                .collect();

            // Get users ids
            let user_ids = match db.get_users_ids_by_emails(&emails).await {
                Ok(user_ids) => user_ids,
                Err(err) => {
                    error!("Failed to get users ids: {:?}", err);
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        CloudApiErrors::DatabaseError.to_string(),
                    ));
                }
            };

            // Get users privileges
            let team_users_privileges = match db.get_privileges_by_team_id(&request.team_id).await {
                Ok(users_in_team_privileges) => users_in_team_privileges,
                Err(err) => {
                    error!("Failed to get users in team privileges: {:?}", err);
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        CloudApiErrors::DatabaseError.to_string(),
                    ));
                }
            };

            // Sort privileges by user_id
            let access_map: HashMap<(&String, &String), &PrivilegeLevel> = team_users_privileges
                .iter()
                .map(|privilege| {
                    (
                        (&privilege.user_id, &privilege.app_id),
                        &privilege.privilege_level,
                    )
                })
                .collect();

            // Start transaction to update users privileges
            let mut tx = db.connection_pool.begin().await.map_err(|err| {
                error!("Failed to start transaction: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    CloudApiErrors::DatabaseError.to_string(),
                )
            })?;

            // Update users privileges
            for requested_change in request.privileges_changes {
                // Determine action
                let new_privilege_level = requested_change.new_privilege_level.to_privilege_level();
                let user_id = user_ids.get(&requested_change.user_email).ok_or((
                    StatusCode::BAD_REQUEST,
                    CloudApiErrors::UserDoesNotExist.to_string(),
                ))?;

                let current_privilege = access_map.get(&(user_id, &requested_change.app_id));

                match &requested_change.new_privilege_level {
                    NewUserPrivilegeLevel::Read => {
                        // check current privilege level
                        match current_privilege {
                            Some(privileges) => {
                                // Update privilege
                                if *privileges == &PrivilegeLevel::Read {
                                    continue;
                                }

                                db.update_user_privilege(
                                    &mut tx,
                                    user_id,
                                    &requested_change.app_id,
                                    // Safe unwrap
                                    new_privilege_level.unwrap(),
                                )
                                .await
                                .map_err(|err| {
                                    error!("Failed to update privilege: {:?}", err);
                                    (
                                        StatusCode::INTERNAL_SERVER_ERROR,
                                        CloudApiErrors::DatabaseError.to_string(),
                                    )
                                })?;
                            }
                            None => {
                                // Check if user has access to the team
                                if !access_map.keys().any(|(user_id, _)| user_id == user_id) {
                                    return Err((
                                        StatusCode::BAD_REQUEST,
                                        CloudApiErrors::UserDoesNotBelongsToTheTeam.to_string(),
                                    ));
                                }

                                // Insert new privilege
                                let user_new_privilege = UserAppPrivilege {
                                    user_id: user_id.clone(),
                                    app_id: requested_change.app_id.clone(),
                                    privilege_level: PrivilegeLevel::Read,
                                    creation_timestamp: get_current_datetime(),
                                };

                                db.add_new_privilege_within_tx(&mut tx, &user_new_privilege)
                                    .await
                                    .map_err(|err| {
                                        error!("Failed to add new privilege: {:?}", err);
                                        (
                                            StatusCode::INTERNAL_SERVER_ERROR,
                                            CloudApiErrors::DatabaseError.to_string(),
                                        )
                                    })?;
                            }
                        };
                    }
                    NewUserPrivilegeLevel::Edit => {
                        // check current privilege level
                        match current_privilege {
                            Some(privileges) => {
                                // Update privilege
                                if *privileges == &PrivilegeLevel::Edit {
                                    continue;
                                }

                                db.update_user_privilege(
                                    &mut tx,
                                    user_id,
                                    &requested_change.app_id,
                                    // Safe unwrap
                                    new_privilege_level.unwrap(),
                                )
                                .await
                                .map_err(|err| {
                                    error!("Failed to update privilege: {:?}", err);
                                    (
                                        StatusCode::INTERNAL_SERVER_ERROR,
                                        CloudApiErrors::DatabaseError.to_string(),
                                    )
                                })?;
                            }
                            None => {
                                // Check if user has access to the team
                                if !access_map.keys().any(|(user_id, _)| user_id == user_id) {
                                    return Err((
                                        StatusCode::BAD_REQUEST,
                                        CloudApiErrors::UserDoesNotBelongsToTheTeam.to_string(),
                                    ));
                                }

                                // Insert new privilege
                                let user_new_privilege = UserAppPrivilege {
                                    user_id: user_id.clone(),
                                    app_id: requested_change.app_id.clone(),
                                    privilege_level: PrivilegeLevel::Edit,
                                    creation_timestamp: get_current_datetime(),
                                };

                                db.add_new_privilege_within_tx(&mut tx, &user_new_privilege)
                                    .await
                                    .map_err(|err| {
                                        error!("Failed to add new privilege: {:?}", err);
                                        (
                                            StatusCode::INTERNAL_SERVER_ERROR,
                                            CloudApiErrors::DatabaseError.to_string(),
                                        )
                                    })?;
                            }
                        };
                    }
                    NewUserPrivilegeLevel::NoAccess => {
                        // check current privilege level
                        if let Some(_) = current_privilege {
                            // Check if user still has access to the team, if there is entry in the access map to remove access to specific app
                            // then there should be at least one another entry in the access map which will give him access to the team
                            if access_map.keys().any(|(user_id, app_id)| {
                                user_id == user_id && *app_id != &requested_change.app_id
                            }) {
                                // Update privilege
                                db.remove_user_privilege(
                                    &mut tx,
                                    user_id,
                                    &requested_change.app_id,
                                )
                                .await
                                .map_err(|err| {
                                    error!("Failed to remove privilege: {:?}", err);
                                    (
                                        StatusCode::INTERNAL_SERVER_ERROR,
                                        CloudApiErrors::DatabaseError.to_string(),
                                    )
                                })?;
                            }
                        }
                    }
                }
            }

            // Commit transaction
            tx.commit().await.map_err(|err| {
                error!("Failed to commit transaction: {:?}", err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    CloudApiErrors::DatabaseError.to_string(),
                )
            })?;

            // Return response
            return Ok(Json(HttpChangeUsersPrivilegesResponse {}));
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
        http::cloud::{
            get_team_users_privileges::HttpGetTeamUsersPrivilegesResponse,
            register_new_app::HttpRegisterNewAppRequest,
        },
        structs::cloud::{
            cloud_http_endpoints::HttpCloudEndpoint, team_user_privilege::TeamUserPrivilege,
        },
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
    async fn test_change_privileges() {
        let test_app = create_test_app(false).await;

        let (auth_token, _email, _password) = register_and_login_random_user(&test_app).await;

        let team_name = generate_valid_name();
        let team_id = add_test_team(&team_name, &auth_token, &test_app, false)
            .await
            .unwrap();

        let mut app_ids = Vec::new();
        for _ in 0..3 {
            let app_name = generate_valid_name();
            let request = HttpRegisterNewAppRequest {
                team_id: team_id.clone(),
                app_name: app_name.clone(),
            };
            let app_id = add_test_app(&request, &auth_token, &test_app)
                .await
                .unwrap();
            app_ids.push(app_id);
        }

        let mut users_email = Vec::new();
        // Register 10 users and invite them to the team
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

        // Prepare list of changes to be applied
        let mut changes = Vec::new();
        // Change state of 4 users in first
        for i in 0..4 {
            let privilege = if i % 2 == 0 {
                NewUserPrivilegeLevel::Read
            } else {
                NewUserPrivilegeLevel::Edit
            };

            let change = PrivilegeChange {
                app_id: app_ids[0].clone(),
                user_email: users_email[i].clone(),
                new_privilege_level: privilege,
            };

            changes.push(change);
        }
        // Remove access to the second app for the first user
        let change = PrivilegeChange {
            app_id: app_ids[1].clone(),
            user_email: users_email[0].clone(),
            new_privilege_level: NewUserPrivilegeLevel::NoAccess,
        };
        changes.push(change);

        // Try to change privileges
        let request = HttpChangeUsersPrivilegesRequest {
            team_id: team_id.clone(),
            privileges_changes: changes,
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
                HttpCloudEndpoint::ChangeUserPrivileges.to_string()
            ))
            .extension(ip.clone())
            .body(Body::from(json))
            .unwrap();

        // Send request
        let response = test_app.clone().oneshot(req).await.unwrap();
        // Validate response
        convert_response::<HttpChangeUsersPrivilegesResponse>(response)
            .await
            .unwrap();

        // Get users privileges
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

        // Check amount
        assert_eq!(res.users_privileges.len(), 32); // 10 for each user per app + 3 for admin entries - 1 for removed access

        // Check changed privileges to first app
        for i in 0..4 {
            let user_privileges = res
                .users_privileges
                .iter()
                .filter(|x| x.user_email == users_email[i] && x.app_id == app_ids[0])
                .collect::<Vec<&TeamUserPrivilege>>();

            assert_eq!(user_privileges.len(), 1);
            let privilege = user_privileges[0];
            if i % 2 == 0 {
                assert_eq!(privilege.privilege, PrivilegeLevel::Read);
            } else {
                assert_eq!(privilege.privilege, PrivilegeLevel::Edit);
            }
        }

        // Check removed access to the second app
        let user_privileges = res
            .users_privileges
            .iter()
            .filter(|x| x.user_email == users_email[0] && x.app_id == app_ids[1])
            .collect::<Vec<&TeamUserPrivilege>>();
        assert_eq!(user_privileges.len(), 0);

        // Check unchanged privileges to the last app
        for i in 0..10 {
            let user_privileges = res
                .users_privileges
                .iter()
                .filter(|x| x.user_email == users_email[i] && x.app_id == app_ids[2])
                .collect::<Vec<&TeamUserPrivilege>>();

            assert_eq!(user_privileges.len(), 1);
            let privilege = user_privileges[0];
            assert_eq!(privilege.privilege, PrivilegeLevel::Read);
        }
    }
}
