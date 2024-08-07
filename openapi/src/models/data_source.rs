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
pub struct DataSource {
    #[serde(rename = "access", skip_serializing_if = "Option::is_none")]
    pub access: Option<String>,
    /// Metadata contains user accesses for a given resource Ex: map[string]bool{\"create\":true, \"delete\": true}
    #[serde(rename = "accessControl", skip_serializing_if = "Option::is_none")]
    pub access_control: Option<std::collections::HashMap<String, bool>>,
    #[serde(rename = "basicAuth", skip_serializing_if = "Option::is_none")]
    pub basic_auth: Option<bool>,
    #[serde(rename = "basicAuthUser", skip_serializing_if = "Option::is_none")]
    pub basic_auth_user: Option<String>,
    #[serde(rename = "database", skip_serializing_if = "Option::is_none")]
    pub database: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "isDefault", skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(rename = "jsonData", skip_serializing_if = "Option::is_none")]
    pub json_data: Option<serde_json::Value>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "orgId", skip_serializing_if = "Option::is_none")]
    pub org_id: Option<i64>,
    #[serde(rename = "readOnly", skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    #[serde(rename = "secureJsonFields", skip_serializing_if = "Option::is_none")]
    pub secure_json_fields: Option<std::collections::HashMap<String, bool>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "typeLogoUrl", skip_serializing_if = "Option::is_none")]
    pub type_logo_url: Option<String>,
    #[serde(rename = "uid", skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "user", skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(rename = "version", skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[serde(rename = "withCredentials", skip_serializing_if = "Option::is_none")]
    pub with_credentials: Option<bool>,
}

impl DataSource {
    pub fn new() -> DataSource {
        DataSource {
            access: None,
            access_control: None,
            basic_auth: None,
            basic_auth_user: None,
            database: None,
            id: None,
            is_default: None,
            json_data: None,
            name: None,
            org_id: None,
            read_only: None,
            secure_json_fields: None,
            r#type: None,
            type_logo_url: None,
            uid: None,
            url: None,
            user: None,
            version: None,
            with_credentials: None,
        }
    }
}
