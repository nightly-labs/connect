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
pub struct SyncResult {
    /// A Duration represents the elapsed time between two instants as an int64 nanosecond count. The representation limits the largest representable duration to approximately 290 years.
    #[serde(rename = "Elapsed", skip_serializing_if = "Option::is_none")]
    pub elapsed: Option<i64>,
    #[serde(rename = "FailedUsers", skip_serializing_if = "Option::is_none")]
    pub failed_users: Option<Vec<models::FailedUser>>,
    #[serde(rename = "MissingUserIds", skip_serializing_if = "Option::is_none")]
    pub missing_user_ids: Option<Vec<i64>>,
    #[serde(rename = "Started", skip_serializing_if = "Option::is_none")]
    pub started: Option<String>,
    #[serde(rename = "UpdatedUserIds", skip_serializing_if = "Option::is_none")]
    pub updated_user_ids: Option<Vec<i64>>,
}

impl SyncResult {
    pub fn new() -> SyncResult {
        SyncResult {
            elapsed: None,
            failed_users: None,
            missing_user_ids: None,
            started: None,
            updated_user_ids: None,
        }
    }
}
