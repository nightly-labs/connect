use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};
use ts_rs::TS;
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
    Serialize,
    Deserialize,
    TS,
    Display,
    EnumIter,
    EnumString,
)]
#[ts(export)]
pub enum NightlyError {
    RequestTimeOut,
    ServiceOverload,
    UnhandledInternalError,
    SessionDoesNotExist,
    AppDisconnected,
    UserNotConnected,
    SessionDropped,
    RequestDoesNotExist,
    NotFound,
    ClientAlreadyInitialized,
}
