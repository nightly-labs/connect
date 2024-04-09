use database::structs::privilege_level::PrivilegeLevel;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub enum NewUserPrivilegeLevel {
    Read,
    Write,
    NoAccess,
}

impl NewUserPrivilegeLevel {
    pub fn to_privilege_level(&self) -> Option<PrivilegeLevel> {
        match self {
            NewUserPrivilegeLevel::Read => Some(PrivilegeLevel::Read),
            NewUserPrivilegeLevel::Write => Some(PrivilegeLevel::Edit),
            NewUserPrivilegeLevel::NoAccess => None,
        }
    }
}
