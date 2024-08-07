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
pub struct ProxyConfig {
    /// NoProxy contains addresses that should not use a proxy.
    #[serde(rename = "no_proxy", skip_serializing_if = "Option::is_none")]
    pub no_proxy: Option<String>,
    #[serde(rename = "proxy_connect_header", skip_serializing_if = "Option::is_none")]
    pub proxy_connect_header: Option<std::collections::HashMap<String, Vec<String>>>,
    /// ProxyFromEnvironment makes use of net/http ProxyFromEnvironment function to determine proxies.
    #[serde(rename = "proxy_from_environment", skip_serializing_if = "Option::is_none")]
    pub proxy_from_environment: Option<bool>,
    #[serde(rename = "proxy_url", skip_serializing_if = "Option::is_none")]
    pub proxy_url: Option<Box<models::Url>>,
}

impl ProxyConfig {
    pub fn new() -> ProxyConfig {
        ProxyConfig {
            no_proxy: None,
            proxy_connect_header: None,
            proxy_from_environment: None,
            proxy_url: None,
        }
    }
}
