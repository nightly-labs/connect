use crate::state::AppId;
use database::structs::privilege_level::PrivilegeLevel;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct TeamUserPrivilege {
    pub app_id: AppId,
    pub user_email: String,
    pub privilege: PrivilegeLevel,
}
