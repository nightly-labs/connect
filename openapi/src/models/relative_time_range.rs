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

/// RelativeTimeRange : RelativeTimeRange is the per query start and end time for requests.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RelativeTimeRange {
    /// A Duration represents the elapsed time between two instants as an int64 nanosecond count. The representation limits the largest representable duration to approximately 290 years.
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    /// A Duration represents the elapsed time between two instants as an int64 nanosecond count. The representation limits the largest representable duration to approximately 290 years.
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
}

impl RelativeTimeRange {
    /// RelativeTimeRange is the per query start and end time for requests.
    pub fn new() -> RelativeTimeRange {
        RelativeTimeRange {
            from: None,
            to: None,
        }
    }
}

