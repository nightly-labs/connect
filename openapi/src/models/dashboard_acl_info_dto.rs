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
pub struct DashboardAclInfoDto {
    #[serde(rename = "created", skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    #[serde(rename = "dashboardId", skip_serializing_if = "Option::is_none")]
    pub dashboard_id: Option<i64>,
    /// Deprecated: use FolderUID instead
    #[serde(rename = "folderId", skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<i64>,
    #[serde(rename = "folderUid", skip_serializing_if = "Option::is_none")]
    pub folder_uid: Option<String>,
    #[serde(rename = "inherited", skip_serializing_if = "Option::is_none")]
    pub inherited: Option<bool>,
    #[serde(rename = "isFolder", skip_serializing_if = "Option::is_none")]
    pub is_folder: Option<bool>,
    #[serde(rename = "permission", skip_serializing_if = "Option::is_none")]
    pub permission: Option<i64>,
    #[serde(rename = "permissionName", skip_serializing_if = "Option::is_none")]
    pub permission_name: Option<String>,
    #[serde(rename = "role", skip_serializing_if = "Option::is_none")]
    pub role: Option<Role>,
    #[serde(rename = "slug", skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    #[serde(rename = "team", skip_serializing_if = "Option::is_none")]
    pub team: Option<String>,
    #[serde(rename = "teamAvatarUrl", skip_serializing_if = "Option::is_none")]
    pub team_avatar_url: Option<String>,
    #[serde(rename = "teamEmail", skip_serializing_if = "Option::is_none")]
    pub team_email: Option<String>,
    #[serde(rename = "teamId", skip_serializing_if = "Option::is_none")]
    pub team_id: Option<i64>,
    #[serde(rename = "title", skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "uid", skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
    #[serde(rename = "updated", skip_serializing_if = "Option::is_none")]
    pub updated: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "userAvatarUrl", skip_serializing_if = "Option::is_none")]
    pub user_avatar_url: Option<String>,
    #[serde(rename = "userEmail", skip_serializing_if = "Option::is_none")]
    pub user_email: Option<String>,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(rename = "userLogin", skip_serializing_if = "Option::is_none")]
    pub user_login: Option<String>,
}

impl DashboardAclInfoDto {
    pub fn new() -> DashboardAclInfoDto {
        DashboardAclInfoDto {
            created: None,
            dashboard_id: None,
            folder_id: None,
            folder_uid: None,
            inherited: None,
            is_folder: None,
            permission: None,
            permission_name: None,
            role: None,
            slug: None,
            team: None,
            team_avatar_url: None,
            team_email: None,
            team_id: None,
            title: None,
            uid: None,
            updated: None,
            url: None,
            user_avatar_url: None,
            user_email: None,
            user_id: None,
            user_login: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Role {
    #[serde(rename = "None")]
    None,
    #[serde(rename = "Viewer")]
    Viewer,
    #[serde(rename = "Editor")]
    Editor,
    #[serde(rename = "Admin")]
    Admin,
}

impl Default for Role {
    fn default() -> Role {
        Self::None
    }
}
