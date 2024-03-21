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

impl From<&Option<RequestFail>> for RequestStatus {
    fn from(fail: &Option<RequestFail>) -> Self {
        match fail {
            Some(RequestFail::Rejected) => RequestStatus::Rejected,
            Some(RequestFail::TimedOut) => RequestStatus::TimedOut,
            None => RequestStatus::Completed,
        }
    }
}
