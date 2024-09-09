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

/// DashboardSnapshotDto : DashboardSnapshotDTO without dashboard map
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DashboardSnapshotDto {
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "expires", skip_serializing_if = "Option::is_none")]
    pub expires: Option<String>,
    #[serde(rename = "external", skip_serializing_if = "Option::is_none")]
    pub external: Option<bool>,
    #[serde(rename = "externalUrl", skip_serializing_if = "Option::is_none")]
    pub external_url: Option<String>,
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "updated", skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
}

impl DashboardSnapshotDto {
    /// DashboardSnapshotDTO without dashboard map
    pub fn new() -> DashboardSnapshotDto {
        DashboardSnapshotDto {
            created: None,
            expires: None,
            external: None,
            external_url: None,
            key: None,
            name: None,
            updated: None,
        }
    }
}
