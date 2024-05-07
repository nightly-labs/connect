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
pub struct SearchOrgUsersQueryResult {
    #[serde(rename = "orgUsers", skip_serializing_if = "Option::is_none")]
    pub org_users: Option<Vec<models::OrgUserDto>>,
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(rename = "perPage", skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
    #[serde(rename = "totalCount", skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}

impl SearchOrgUsersQueryResult {
    pub fn new() -> SearchOrgUsersQueryResult {
        SearchOrgUsersQueryResult {
            org_users: None,
            page: None,
            per_page: None,
            total_count: None,
        }
    }
}

