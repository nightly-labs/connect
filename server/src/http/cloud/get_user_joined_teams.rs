use crate::{
    auth::auth_middleware::UserId,
    state::AppId,
    structs::{
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

                teams_apps.insert(team_id.clone(), apps_info);
                user_privileges.insert(team_id, privileges);
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
