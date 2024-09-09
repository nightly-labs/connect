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
pub struct EmailConfig {
    #[serde(rename = "auth_identity", skip_serializing_if = "Option::is_none")]
    pub auth_identity: Option<String>,
    #[serde(rename = "auth_password", skip_serializing_if = "Option::is_none")]
    pub auth_password: Option<String>,
    #[serde(rename = "auth_password_file", skip_serializing_if = "Option::is_none")]
    pub auth_password_file: Option<String>,
    #[serde(rename = "auth_secret", skip_serializing_if = "Option::is_none")]
    pub auth_secret: Option<String>,
    #[serde(rename = "auth_username", skip_serializing_if = "Option::is_none")]
    pub auth_username: Option<String>,
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(rename = "headers", skip_serializing_if = "Option::is_none")]
    pub headers: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "hello", skip_serializing_if = "Option::is_none")]
    pub hello: Option<String>,
    #[serde(rename = "html", skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
    #[serde(rename = "require_tls", skip_serializing_if = "Option::is_none")]
    pub require_tls: Option<bool>,
    #[serde(rename = "send_resolved", skip_serializing_if = "Option::is_none")]
    pub send_resolved: Option<bool>,
    #[serde(rename = "smarthost", skip_serializing_if = "Option::is_none")]
    pub smarthost: Option<Box<models::HostPort>>,
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(rename = "tls_config", skip_serializing_if = "Option::is_none")]
    pub tls_config: Option<Box<models::TlsConfig>>,
    /// Email address to notify.
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}

impl EmailConfig {
    pub fn new() -> EmailConfig {
        EmailConfig {
            auth_identity: None,
            auth_password: None,
            auth_password_file: None,
            auth_secret: None,
            auth_username: None,
            from: None,
            headers: None,
            hello: None,
            html: None,
            require_tls: None,
            send_resolved: None,
            smarthost: None,
            text: None,
            tls_config: None,
            to: None,
        }
    }
}
