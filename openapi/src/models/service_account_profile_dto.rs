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
pub struct ServiceAccountProfileDto {
    #[serde(rename = "accessControl", skip_serializing_if = "Option::is_none")]
    pub access_control: Option<std::collections::HashMap<String, bool>>,
    #[serde(rename = "avatarUrl", skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    #[serde(rename = "createdAt", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "isDisabled", skip_serializing_if = "Option::is_none")]
    pub is_disabled: Option<bool>,
    #[serde(rename = "isExternal", skip_serializing_if = "Option::is_none")]
    pub is_external: Option<bool>,
    #[serde(rename = "login", skip_serializing_if = "Option::is_none")]
    pub login: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "orgId", skip_serializing_if = "Option::is_none")]
    pub org_id: Option<i64>,
    #[serde(rename = "requiredBy", skip_serializing_if = "Option::is_none")]
    pub required_by: Option<String>,
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(rename = "teams", skip_serializing_if = "Option::is_none")]
    pub teams: Option<Vec<String>>,
    #[serde(rename = "tokens", skip_serializing_if = "Option::is_none")]
    pub tokens: Option<i64>,
    #[serde(rename = "updatedAt", skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

impl ServiceAccountProfileDto {
    pub fn new() -> ServiceAccountProfileDto {
        ServiceAccountProfileDto {
            access_control: None,
            avatar_url: None,
            created_at: None,
            id: None,
            is_disabled: None,
            is_external: None,
            login: None,
            name: None,
            org_id: None,
            required_by: None,
            role: None,
            teams: None,
            tokens: None,
            updated_at: None,
        }
    }
}

