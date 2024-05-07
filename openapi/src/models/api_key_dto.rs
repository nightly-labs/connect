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
pub struct ApiKeyDto {
    /// Metadata contains user accesses for a given resource Ex: map[string]bool{\"create\":true, \"delete\": true}
    #[serde(rename = "accessControl", skip_serializing_if = "Option::is_none")]
    pub access_control: Option<std::collections::HashMap<String, bool>>,
    #[serde(rename = "expiration", skip_serializing_if = "Option::is_none")]
    pub expiration: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "lastUsedAt", skip_serializing_if = "Option::is_none")]
    pub last_used_at: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
}

impl ApiKeyDto {
    pub fn new() -> ApiKeyDto {
        ApiKeyDto {
            access_control: None,
            expiration: None,
            id: None,
            last_used_at: None,
            name: None,
            role: None,
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

