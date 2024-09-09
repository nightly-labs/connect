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
pub struct PlaylistDto {
    /// Interval sets the time between switching views in a playlist.
    #[serde(rename = "interval", skip_serializing_if = "Option::is_none")]
    pub interval: Option<String>,
    /// The ordered list of items that the playlist will iterate over.
    #[serde(rename = "items", skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<models::PlaylistItemDto>>,
    /// Name of the playlist.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Unique playlist identifier. Generated on creation, either by the creator of the playlist of by the application.
    #[serde(rename = "uid", skip_serializing_if = "Option::is_none")]
    pub uid: Option<String>,
}

impl PlaylistDto {
    pub fn new() -> PlaylistDto {
        PlaylistDto {
            interval: None,
            items: None,
            name: None,
            uid: None,
        }
    }
}
