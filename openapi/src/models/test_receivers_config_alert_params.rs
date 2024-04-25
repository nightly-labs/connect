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
pub struct TestReceiversConfigAlertParams {
    /// A LabelSet is a collection of LabelName and LabelValue pairs.  The LabelSet may be fully-qualified down to the point where it may resolve to a single Metric in the data store or not.  All operations that occur within the realm of a LabelSet can emit a vector of Metric entities to which the LabelSet may match.
    #[serde(rename = "annotations", skip_serializing_if = "Option::is_none")]
    pub annotations: Option<std::collections::HashMap<String, String>>,
    /// A LabelSet is a collection of LabelName and LabelValue pairs.  The LabelSet may be fully-qualified down to the point where it may resolve to a single Metric in the data store or not.  All operations that occur within the realm of a LabelSet can emit a vector of Metric entities to which the LabelSet may match.
    #[serde(rename = "labels", skip_serializing_if = "Option::is_none")]
    pub labels: Option<std::collections::HashMap<String, String>>,
}

impl TestReceiversConfigAlertParams {
    pub fn new() -> TestReceiversConfigAlertParams {
        TestReceiversConfigAlertParams {
            annotations: None,
            labels: None,
        }
    }
}

