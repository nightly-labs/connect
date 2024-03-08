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
    #[serde(rename = "/register_new_team")]
    RegisterNewTeam,
    #[serde(rename = "/add_user_to_team")]
    AddUserToTeam,
    #[serde(rename = "/remove_user_from_team")]
    RemoveUserFromTeam,
}

impl HttpCloudEndpoint {
    pub fn to_string(&self) -> String {
        match self {
            HttpCloudEndpoint::RegisterNewApp => "/register_new_app".to_string(),
            HttpCloudEndpoint::RegisterWithPassword => "/register_with_password".to_string(),
            HttpCloudEndpoint::LoginWithPassword => "/login_with_password".to_string(),
            HttpCloudEndpoint::RegisterNewTeam => "/register_new_team".to_string(),
            HttpCloudEndpoint::AddUserToTeam => "/add_user_to_team".to_string(),
            HttpCloudEndpoint::RemoveUserFromTeam => "/remove_user_from_team".to_string(),
        }
    }
}
