use crate::state::AppId;
use chrono::{DateTime, Utc};
use database::{
    structs::privilege_level::PrivilegeLevel,
    tables::user_app_privileges::table_struct::UserAppPrivilege,
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct UserPrivilege {
    pub app_id: AppId,
    pub granted_at: DateTime<Utc>,
    pub privilege: PrivilegeLevel,
}

impl From<UserAppPrivilege> for UserPrivilege {
    fn from(user_app_privilege: UserAppPrivilege) -> Self {
        UserPrivilege {
            app_id: user_app_privilege.app_id,
            granted_at: user_app_privilege.creation_timestamp,
            privilege: user_app_privilege.privilege_level,
        }
    }
}
