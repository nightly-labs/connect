/*
 * Grafana HTTP API.
 *
 * The Grafana backend exposes an HTTP API, the same API is used by the frontend to do everything from saving dashboards, creating users and updating data sources.
 *
 * The version of the OpenAPI document: 0.0.1
 * Contact: hello@grafana.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AddInviteForm {
    #[serde(rename = "loginOrEmail", skip_serializing_if = "Option::is_none")]
    pub login_or_email: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
    #[serde(rename = "sendEmail", skip_serializing_if = "Option::is_none")]
    pub send_email: Option<bool>,
}

impl AddInviteForm {
    pub fn new() -> AddInviteForm {
        AddInviteForm {
            login_or_email: None,
            name: None,
            role: None,
            send_email: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "None")]
    None,
    #[serde(rename = "Viewer")]
    Viewer,
    #[serde(rename = "Editor")]
    Editor,
    #[serde(rename = "Admin")]
    Admin,
}

impl Default for Role {
    fn default() -> Role {
        Self::None
    }
}
