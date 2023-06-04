use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::structs::common::{AckMessage, ErrorMessage};

use super::{
    app_disconnected_event::AppDisconnectedEvent,
    client_initialize::{ClientInitializeRequest, ClientInitializeResponse},
    connect::{ConnectRequest, ConnectResponse},
    get_info::{GetInfoRequest, GetInfoResponse},
    get_pending_requests::{GetPendingRequestsRequest, GetPendingRequestsResponse},
    new_payload_event::{NewPayloadEvent, NewPayloadEventReply},
};

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize, TS)]
#[ts(export)]
#[serde(tag = "type")]
pub enum ClientToServer {
    ClientInitializeRequest(ClientInitializeRequest),
    ConnectRequest(ConnectRequest),
    GetInfoRequest(GetInfoRequest),
    NewPayloadEventReply(NewPayloadEventReply),
    GetPendingRequestsRequest(GetPendingRequestsRequest),
}
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize, TS)]
#[ts(export)]
#[serde(tag = "type")]
pub enum ServerToClient {
    ClientInitializeResponse(ClientInitializeResponse),
    GetInfoResponse(GetInfoResponse),
    ConnectResponse(ConnectResponse),
    NewPayloadEvent(NewPayloadEvent),
    AppDisconnectedEvent(AppDisconnectedEvent),
    GetPendingRequestsResponse(GetPendingRequestsResponse),
    ErrorMessage(ErrorMessage),
    AckMessage(AckMessage),
}
