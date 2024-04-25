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
pub struct PublicDashboardDto {
    #[serde(rename = "accessToken", skip_serializing_if = "Option::is_none")]
    pub access_token: Option<String>,
    #[serde(rename = "annotationsEnabled", skip_serializing_if = "Option::is_none")]
    pub annotations_enabled: Option<bool>,
    #[serde(rename = "isEnabled", skip_serializing_if = "Option::is_none")]
    pub is_enabled: Option<bool>,
    #[serde(rename = "share", skip_serializing_if = "Option::is_none")]
    pub share: Option<String>,
    #[serde(rename = "timeSelectionEnabled", skip_serializing_if = "Option::is_none")]
    pub time_selection_enabled: Option<bool>,
    #[serde(rename = "uid", skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

impl PublicDashboardDto {
    pub fn new() -> PublicDashboardDto {
        PublicDashboardDto {
            access_token: None,
            annotations_enabled: None,
            is_enabled: None,
            share: None,
            time_selection_enabled: None,
            uid: None,
        }
    }
}

