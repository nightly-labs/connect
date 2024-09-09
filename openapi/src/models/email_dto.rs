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
pub struct EmailDto {
    #[serde(rename = "recipient", skip_serializing_if = "Option::is_none")]
    pub recipient: Option<String>,
    #[serde(rename = "uid", skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

impl EmailDto {
    pub fn new() -> EmailDto {
        EmailDto {
            recipient: None,
            uid: None,
        }
    }
}
