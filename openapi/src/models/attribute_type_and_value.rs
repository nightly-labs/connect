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

/// AttributeTypeAndValue : AttributeTypeAndValue mirrors the ASN.1 structure of the same name in RFC 5280, Section 4.1.2.4.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AttributeTypeAndValue {
    #[serde(rename = "Type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Vec<i64>>,
    #[serde(rename = "Value", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub value: Option<Option<serde_json::Value>>,
}

impl AttributeTypeAndValue {
    /// AttributeTypeAndValue mirrors the ASN.1 structure of the same name in RFC 5280, Section 4.1.2.4.
    pub fn new() -> AttributeTypeAndValue {
        AttributeTypeAndValue {
            r#type: None,
            value: None,
        }
    }
}
