use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::structs::common::{AckMessage, ErrorMessage};

use super::{
    already_connected::AlreadyConnected,
    initialize::{InitializeRequest, InitializeResponse},
    payload::{RequestPayload, ResponsePayload},
    user_connected_event::UserConnectedEvent,
    user_disconnected_event::UserDisconnectedEvent,
};

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize, TS)]
#[ts(export)]
#[serde(tag = "type")]
pub enum AppToServer {
    InitializeRequest(InitializeRequest),
    RequestPayload(RequestPayload),
}
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize, TS)]
#[ts(export)]
#[serde(tag = "type")]
pub enum ServerToApp {
    InitializeResponse(InitializeResponse),
    UserConnectedEvent(UserConnectedEvent),
    UserDisconnectedEvent(UserDisconnectedEvent),
    ResponsePayload(ResponsePayload),
    ErrorMessage(ErrorMessage),
    AckMessage(AckMessage),
    AlreadyConnected(AlreadyConnected),
}
