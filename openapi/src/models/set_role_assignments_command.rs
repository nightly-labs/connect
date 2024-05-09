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
pub struct SetRoleAssignmentsCommand {
    #[serde(rename = "service_accounts", skip_serializing_if = "Option::is_none")]
    pub service_accounts: Option<Vec<i64>>,
    #[serde(rename = "teams", skip_serializing_if = "Option::is_none")]
    pub teams: Option<Vec<i64>>,
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<i64>>,
}

impl SetRoleAssignmentsCommand {
    pub fn new() -> SetRoleAssignmentsCommand {
        SetRoleAssignmentsCommand {
            service_accounts: None,
            teams: None,
            users: None,
        }
    }
}

