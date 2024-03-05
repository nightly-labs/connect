use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Clone, Debug, Eq, PartialEq, Type, Serialize, Deserialize)]
#[sqlx(type_name = "subscription")]
pub struct Subscription {
    pub subscription_type: String,
    pub valid_from: i64,
    pub valid_till: i64,
}
