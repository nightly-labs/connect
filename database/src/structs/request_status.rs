use serde::{Deserialize, Serialize};
use sqlx::Type;
use ts_rs::TS;

#[derive(Clone, Debug, Eq, PartialEq, Type)]
#[sqlx(type_name = "request_status_enum")]
pub enum RequestStatus {
    Pending,
    Completed,
    Rejected,
    TimedOut,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum RequestFail {
    Rejected,
    TimedOut,
}

impl Into<RequestStatus> for RequestFail {
    fn into(self) -> RequestStatus {
        match self {
            RequestFail::Rejected => RequestStatus::Rejected,
            RequestFail::TimedOut => RequestStatus::TimedOut,
        }
    }
}
