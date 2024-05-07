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
pub struct GettableGrafanaReceiver {
    #[serde(rename = "disableResolveMessage", skip_serializing_if = "Option::is_none")]
    pub disable_resolve_message: Option<bool>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "provenance", skip_serializing_if = "Option::is_none")]
    pub provenance: Option<String>,
    #[serde(rename = "secureFields", skip_serializing_if = "Option::is_none")]
    pub secure_fields: Option<std::collections::HashMap<String, bool>>,
    #[serde(rename = "settings", skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "uid", skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

impl GettableGrafanaReceiver {
    pub fn new() -> GettableGrafanaReceiver {
        GettableGrafanaReceiver {
            disable_resolve_message: None,
            name: None,
            provenance: None,
            secure_fields: None,
            settings: None,
            r#type: None,
            uid: None,
        }
    }
}
