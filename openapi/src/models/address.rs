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
pub struct Address {
    #[serde(rename = "address1", skip_serializing_if = "Option::is_none")]
    pub address1: Option<String>,
    #[serde(rename = "address2", skip_serializing_if = "Option::is_none")]
    pub address2: Option<String>,
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "zipCode", skip_serializing_if = "Option::is_none")]
    pub zip_code: Option<String>,
}

impl Address {
    pub fn new() -> Address {
        Address {
            address1: None,
            address2: None,
            city: None,
            country: None,
            state: None,
            zip_code: None,
        }
    }
}
