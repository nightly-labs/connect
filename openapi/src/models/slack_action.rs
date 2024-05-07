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

/// SlackAction : See https://api.slack.com/docs/message-attachments#action_fields and https://api.slack.com/docs/message-buttons for more information.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SlackAction {
    #[serde(rename = "confirm", skip_serializing_if = "Option::is_none")]
    pub confirm: Option<Box<models::SlackConfirmationField>>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "style", skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl SlackAction {
    /// See https://api.slack.com/docs/message-attachments#action_fields and https://api.slack.com/docs/message-buttons for more information.
    pub fn new() -> SlackAction {
        SlackAction {
            confirm: None,
            name: None,
            style: None,
            text: None,
            r#type: None,
            url: None,
            value: None,
        }
    }
}
