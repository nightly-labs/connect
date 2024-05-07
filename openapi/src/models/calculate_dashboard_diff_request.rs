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
pub struct CalculateDashboardDiffRequest {
    #[serde(rename = "base", skip_serializing_if = "Option::is_none")]
    pub base: Option<Box<models::CalculateDiffTarget>>,
    /// The type of diff to return Description: `basic` `json`
    #[serde(rename = "diffType", skip_serializing_if = "Option::is_none")]
    pub diff_type: Option<DiffType>,
    #[serde(rename = "new", skip_serializing_if = "Option::is_none")]
    pub new: Option<Box<models::CalculateDiffTarget>>,
}

impl CalculateDashboardDiffRequest {
    pub fn new() -> CalculateDashboardDiffRequest {
        CalculateDashboardDiffRequest {
            base: None,
            diff_type: None,
            new: None,
        }
    }
}
/// The type of diff to return Description: `basic` `json`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DiffType {
    #[serde(rename = "basic")]
    Basic,
    #[serde(rename = "json")]
    Json,
}

impl Default for DiffType {
    fn default() -> DiffType {
        Self::Basic
    }
}

