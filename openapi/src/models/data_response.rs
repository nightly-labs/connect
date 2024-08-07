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

/// DataResponse : A map of RefIDs (unique query identifiers) to this type makes up the Responses property of a QueryDataResponse. The Error property is used to allow for partial success responses from the containing QueryDataResponse.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataResponse {
    /// Error is a property to be set if the corresponding DataQuery has an error.
    #[serde(rename = "Error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    /// ErrorSource type defines the source of the error
    #[serde(rename = "ErrorSource", skip_serializing_if = "Option::is_none")]
    pub error_source: Option<String>,
    /// It is the main data container within a backend.DataResponse. There should be no `nil` entries in the Frames slice (making them pointers was a mistake).
    #[serde(rename = "Frames", skip_serializing_if = "Option::is_none")]
    pub frames: Option<Vec<models::Frame>>,
    #[serde(rename = "Status", skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
}

impl DataResponse {
    /// A map of RefIDs (unique query identifiers) to this type makes up the Responses property of a QueryDataResponse. The Error property is used to allow for partial success responses from the containing QueryDataResponse.
    pub fn new() -> DataResponse {
        DataResponse {
            error: None,
            error_source: None,
            frames: None,
            status: None,
        }
    }
}
