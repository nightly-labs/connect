use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum HttpStatsEndpoint {
    #[serde(rename = "/get_registered_apps")]
    GetRegisteredApps,
    #[serde(rename = "/register_new_app")]
    RegisterNewApp,
    #[serde(rename = "/register_new_user")]
    RegisterNewUser,
}

impl HttpStatsEndpoint {
    pub fn to_string(&self) -> String {
        match self {
            HttpStatsEndpoint::GetRegisteredApps => "/get_registered_apps".to_string(),
            HttpStatsEndpoint::RegisterNewApp => "/register_new_app".to_string(),
            HttpStatsEndpoint::RegisterNewUser => "/register_new_user".to_string(),
        }
    }
}
