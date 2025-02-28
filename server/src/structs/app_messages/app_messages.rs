use super::{
    already_connected::AlreadyConnected,
    initialize::{InitializeRequest, InitializeResponse},
    payload::{DisconnectRequest, RequestPayload, ResponsePayload},
    user_connected_event::UserConnectedEvent,
    user_disconnected_event::UserDisconnectedEvent,
};
use crate::structs::common::{AckMessage, ErrorMessage};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize, TS)]
#[ts(export)]
#[serde(tag = "type")]
pub enum AppToServer {
    InitializeRequest(InitializeRequest),
    RequestPayload(RequestPayload),
    DisconnectRequest(DisconnectRequest),
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
