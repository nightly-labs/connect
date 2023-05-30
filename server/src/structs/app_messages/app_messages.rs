use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::structs::common::{AckMessage, ErrorMessage};

use super::{
    initialize::{InitializeRequest, InitializeResponse},
    request_rejected::RequestRejected,
    sign_messages::{SignMessagesRequest, SignMessagesResponse},
    sign_transactions::{SignTransactionsRequest, SignTransactionsResponse},
    user_connected_event::UserConnectedEvent,
    user_disconnected_event::UserDisconnectedEvent,
};

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize, TS)]
#[ts(export)]
#[serde(tag = "type")]
pub enum AppToServer {
    InitializeRequest(InitializeRequest),
    SignTransactionsRequest(SignTransactionsRequest),
    SignMessagesRequest(SignMessagesRequest),
}
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize, TS)]
#[ts(export)]
#[serde(tag = "type")]
pub enum ServerToApp {
    InitializeResponse(InitializeResponse),
    UserConnectedEvent(UserConnectedEvent),
    UserDisconnectedEvent(UserDisconnectedEvent),
    SignTransactionsResponse(SignTransactionsResponse),
    SignMessagesResponse(SignMessagesResponse),
    RequestRejected(RequestRejected),
    ErrorMessage(ErrorMessage),
    AckMessage(AckMessage),
}
