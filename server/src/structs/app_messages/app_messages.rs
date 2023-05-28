use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::{
    initialize::{InitializeRequest, InitializeResponse},
    sign_transactions::SignTransactionsRequest,
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
}
