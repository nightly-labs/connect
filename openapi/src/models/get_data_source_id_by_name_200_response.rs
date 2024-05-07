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
pub struct GetDataSourceIdByName200Response {
    /// ID Identifier of the data source.
    #[serde(rename = "id")]
    pub id: i64,
}

impl GetDataSourceIdByName200Response {
    pub fn new(id: i64) -> GetDataSourceIdByName200Response {
        GetDataSourceIdByName200Response {
            id,
        }
    }
}

