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
pub struct ReportSettings {
    #[serde(rename = "branding", skip_serializing_if = "Option::is_none")]
    pub branding: Option<Box<models::ReportBrandingOptions>>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "orgId", skip_serializing_if = "Option::is_none")]
    pub org_id: Option<i64>,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
}

impl ReportSettings {
    pub fn new() -> ReportSettings {
        ReportSettings {
            branding: None,
            id: None,
            org_id: None,
            user_id: None,
        }
    }
}
