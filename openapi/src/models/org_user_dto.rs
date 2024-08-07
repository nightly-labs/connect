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
pub struct OrgUserDto {
    #[serde(rename = "accessControl", skip_serializing_if = "Option::is_none")]
    pub access_control: Option<std::collections::HashMap<String, bool>>,
    #[serde(rename = "authLabels", skip_serializing_if = "Option::is_none")]
    pub auth_labels: Option<Vec<String>>,
    #[serde(rename = "avatarUrl", skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "isDisabled", skip_serializing_if = "Option::is_none")]
    pub is_disabled: Option<bool>,
    #[serde(rename = "isExternallySynced", skip_serializing_if = "Option::is_none")]
    pub is_externally_synced: Option<bool>,
    #[serde(rename = "lastSeenAt", skip_serializing_if = "Option::is_none")]
    pub last_seen_at: Option<String>,
    #[serde(rename = "lastSeenAtAge", skip_serializing_if = "Option::is_none")]
    pub last_seen_at_age: Option<String>,
    #[serde(rename = "login", skip_serializing_if = "Option::is_none")]
    pub login: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "orgId", skip_serializing_if = "Option::is_none")]
    pub org_id: Option<i64>,
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}

impl OrgUserDto {
    pub fn new() -> OrgUserDto {
        OrgUserDto {
            access_control: None,
            auth_labels: None,
            avatar_url: None,
            email: None,
            is_disabled: None,
            is_externally_synced: None,
            last_seen_at: None,
            last_seen_at_age: None,
            login: None,
            name: None,
            org_id: None,
            role: None,
            user_id: None,
        }
    }
}
