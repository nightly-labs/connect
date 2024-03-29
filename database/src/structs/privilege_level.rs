use serde::{Deserialize, Serialize};
use sqlx::Type;
use ts_rs::TS;

#[derive(Debug, Clone, Eq, PartialEq, Type, Serialize, Deserialize, TS)]
#[ts(export)]
#[sqlx(type_name = "privilege_level_enum")]
pub enum PrivilegeLevel {
    Read,
    Edit,
    Admin,
}
