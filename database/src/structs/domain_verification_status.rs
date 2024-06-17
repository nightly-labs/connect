use serde::{Deserialize, Serialize};
use sqlx::Type;
use ts_rs::TS;

#[derive(Debug, Clone, Eq, PartialEq, Type, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum DomainVerificationStatus {
    Pending,
    Verified,
    Cancelled,
}
