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

/// InternalDataLink : InternalDataLink definition to allow Explore links to be constructed in the backend
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct InternalDataLink {
    #[serde(rename = "datasourceName", skip_serializing_if = "Option::is_none")]
    pub datasource_name: Option<String>,
    #[serde(rename = "datasourceUid", skip_serializing_if = "Option::is_none")]
    pub datasource_uid: Option<String>,
    /// This is an object constructed with the keys as the values of the enum VisType and the value being a bag of properties
    #[serde(rename = "panelsState", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub panels_state: Option<Option<serde_json::Value>>,
    #[serde(rename = "query", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub query: Option<Option<serde_json::Value>>,
    #[serde(rename = "timeRange", skip_serializing_if = "Option::is_none")]
    pub time_range: Option<Box<models::TimeRange>>,
    #[serde(rename = "transformations", skip_serializing_if = "Option::is_none")]
    pub transformations: Option<Vec<models::LinkTransformationConfig>>,
}

impl InternalDataLink {
    /// InternalDataLink definition to allow Explore links to be constructed in the backend
    pub fn new() -> InternalDataLink {
        InternalDataLink {
            datasource_name: None,
            datasource_uid: None,
            panels_state: None,
            query: None,
            time_range: None,
            transformations: None,
        }
    }
}
