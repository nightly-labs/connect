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
}

impl HttpCloudEndpoint {
    pub fn to_string(&self) -> String {
        match self {
            HttpCloudEndpoint::RegisterNewApp => "/register_new_app".to_string(),
            HttpCloudEndpoint::RegisterWithPassword => "/register_with_password".to_string(),
            HttpCloudEndpoint::LoginWithPassword => "/login_with_password".to_string(),
        }
    }
}
