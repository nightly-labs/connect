use super::joined_team::TeamId;
use crate::state::AppId;
use chrono::{DateTime, Utc};
use database::{
    structs::whitelisted_domain::WhitelistedDomain,
    tables::registered_app::table_struct::DbRegisteredApp,
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct AppInfo {
    pub team_id: TeamId,
    pub app_id: AppId,
    pub app_name: String,
    pub registered_at: DateTime<Utc>,
    pub whitelisted_domains: Vec<WhitelistedDomain>,
    pub ack_public_keys: Vec<String>,
}

impl From<DbRegisteredApp> for AppInfo {
    fn from(app_info: DbRegisteredApp) -> Self {
        let whitelisted_domains = app_info.get_whitelisted_domains();

        AppInfo {
            team_id: app_info.team_id,
            app_id: app_info.app_id,
            app_name: app_info.app_name,
            registered_at: app_info.registration_timestamp,
            whitelisted_domains: whitelisted_domains,
            ack_public_keys: app_info.ack_public_keys,
        }
    }
}
