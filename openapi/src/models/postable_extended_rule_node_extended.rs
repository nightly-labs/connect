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
pub struct PostableExtendedRuleNodeExtended {
    #[serde(rename = "folderTitle", skip_serializing_if = "Option::is_none")]
    pub folder_title: Option<String>,
    #[serde(rename = "folderUid", skip_serializing_if = "Option::is_none")]
    pub folder_uid: Option<String>,
    #[serde(rename = "rule")]
    pub rule: Box<models::PostableExtendedRuleNode>,
    #[serde(rename = "ruleGroup", skip_serializing_if = "Option::is_none")]
    pub rule_group: Option<String>,
}

impl PostableExtendedRuleNodeExtended {
    pub fn new(rule: models::PostableExtendedRuleNode) -> PostableExtendedRuleNodeExtended {
        PostableExtendedRuleNodeExtended {
            folder_title: None,
            folder_uid: None,
            rule: Box::new(rule),
            rule_group: None,
        }
    }
}
