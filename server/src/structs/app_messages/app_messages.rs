use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::structs::common::ErrorMessage;

use super::{
    initialize::{InitializeRequest, InitializeResponse},
    sign_transactions::{SignTransactionsRequest, SignTransactionsResponse},
    user_connected_event::UserConnectedEvent,
};

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize, TS)]
#[ts(export)]
#[serde(tag = "type")]
pub enum AppToServer {
    InitializeRequest(InitializeRequest),
    SignTransactionsRequest(SignTransactionsRequest),
}
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize, TS)]
#[ts(export)]
#[serde(tag = "type")]
pub enum ServerToApp {
    InitializeResponse(InitializeResponse),
    UserConnectedEvent(UserConnectedEvent),
    SignTransactionsResponse(SignTransactionsResponse),
    ErrorMessage(ErrorMessage),
}
