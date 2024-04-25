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
pub struct PublicDashboardListResponseWithPagination {
    #[serde(rename = "page", skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(rename = "perPage", skip_serializing_if = "Option::is_none")]
    pub per_page: Option<i64>,
    #[serde(rename = "publicDashboards", skip_serializing_if = "Option::is_none")]
    pub public_dashboards: Option<Vec<models::PublicDashboardListResponse>>,
    #[serde(rename = "totalCount", skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
}

impl PublicDashboardListResponseWithPagination {
    pub fn new() -> PublicDashboardListResponseWithPagination {
        PublicDashboardListResponseWithPagination {
            page: None,
            per_page: None,
            public_dashboards: None,
            total_count: None,
        }
    }
}

