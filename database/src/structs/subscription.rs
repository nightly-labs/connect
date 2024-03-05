use serde::{Deserialize, Serialize};
use sqlx::Type;
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS, Type)]
#[ts(export)]
#[sqlx(type_name = "subscription")]
pub struct Subscription {
    pub subscription_type: String,
    pub valid_from: i64,
    pub valid_till: i64,
}
