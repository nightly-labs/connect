use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum HttpCloudEndpoint {
    #[serde(rename = "/register_new_app")]
    RegisterNewApp,
    #[serde(rename = "/register_with_password")]
    RegisterWithPassword,
    #[serde(rename = "/login_with_password")]
    LoginWithPassword,
    #[serde(rename = "/login_with_google")]
    LoginWithGoogle,
    #[serde(rename = "/register_new_team")]
    RegisterNewTeam,
    #[serde(rename = "/remove_user_from_team")]
    RemoveUserFromTeam,
    #[serde(rename = "/get_user_joined_teams")]
    GetUserJoinedTeams,
    #[serde(rename = "/events")]
    Events,
    #[serde(rename = "/invite_user_to_team")]
    InviteUserToTeam,
}

impl HttpCloudEndpoint {
    pub fn to_string(&self) -> String {
        match self {
            HttpCloudEndpoint::RegisterNewApp => "/register_new_app".to_string(),
            HttpCloudEndpoint::RegisterWithPassword => "/register_with_password".to_string(),
            HttpCloudEndpoint::LoginWithPassword => "/login_with_password".to_string(),
            HttpCloudEndpoint::LoginWithGoogle => "/login_with_google".to_string(),
            HttpCloudEndpoint::RegisterNewTeam => "/register_new_team".to_string(),
            HttpCloudEndpoint::RemoveUserFromTeam => "/remove_user_from_team".to_string(),
            HttpCloudEndpoint::GetUserJoinedTeams => "/get_user_joined_teams".to_string(),
            HttpCloudEndpoint::Events => "/events".to_string(),
            HttpCloudEndpoint::InviteUserToTeam => "/invite_user_to_team".to_string(),
        }
    }
}
