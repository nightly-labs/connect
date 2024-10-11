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
pub struct NotificationPolicyExport {
    #[serde(rename = "continue", skip_serializing_if = "Option::is_none")]
    pub r#continue: Option<bool>,
    #[serde(rename = "group_by", skip_serializing_if = "Option::is_none")]
    pub group_by: Option<Vec<String>>,
    #[serde(rename = "group_interval", skip_serializing_if = "Option::is_none")]
    pub group_interval: Option<String>,
    #[serde(rename = "group_wait", skip_serializing_if = "Option::is_none")]
    pub group_wait: Option<String>,
    /// Deprecated. Remove before v1.0 release.
    #[serde(rename = "match", skip_serializing_if = "Option::is_none")]
    pub r#match: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "match_re", skip_serializing_if = "Option::is_none")]
    pub match_re: Option<std::collections::HashMap<String, String>>,
    /// Matchers is a slice of Matchers that is sortable, implements Stringer, and provides a Matches method to match a LabelSet against all Matchers in the slice. Note that some users of Matchers might require it to be sorted.
    #[serde(rename = "matchers", skip_serializing_if = "Option::is_none")]
    pub matchers: Option<Vec<models::Matcher>>,
    #[serde(rename = "mute_time_intervals", skip_serializing_if = "Option::is_none")]
    pub mute_time_intervals: Option<Vec<String>>,
    #[serde(rename = "object_matchers", skip_serializing_if = "Option::is_none")]
    pub object_matchers: Option<Vec<Vec<String>>>,
    #[serde(rename = "orgId", skip_serializing_if = "Option::is_none")]
    pub org_id: Option<i64>,
    #[serde(rename = "receiver", skip_serializing_if = "Option::is_none")]
    pub receiver: Option<String>,
    #[serde(rename = "repeat_interval", skip_serializing_if = "Option::is_none")]
    pub repeat_interval: Option<String>,
    #[serde(rename = "routes", skip_serializing_if = "Option::is_none")]
    pub routes: Option<Vec<models::RouteExport>>,
}

impl NotificationPolicyExport {
    pub fn new() -> NotificationPolicyExport {
        NotificationPolicyExport {
            r#continue: None,
            group_by: None,
            group_interval: None,
            group_wait: None,
            r#match: None,
            match_re: None,
            matchers: None,
            mute_time_intervals: None,
            object_matchers: None,
            org_id: None,
            receiver: None,
            repeat_interval: None,
            routes: None,
        }
    }
}
