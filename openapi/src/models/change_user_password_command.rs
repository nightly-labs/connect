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
pub struct ChangeUserPasswordCommand {
    #[serde(rename = "newPassword", skip_serializing_if = "Option::is_none")]
    pub new_password: Option<String>,
    #[serde(rename = "oldPassword", skip_serializing_if = "Option::is_none")]
    pub old_password: Option<String>,
}

impl ChangeUserPasswordCommand {
    pub fn new() -> ChangeUserPasswordCommand {
        ChangeUserPasswordCommand {
            new_password: None,
            old_password: None,
        }
    }
}
