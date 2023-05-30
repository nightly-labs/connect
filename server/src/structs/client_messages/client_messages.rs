use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::structs::common::{AckMessage, ErrorMessage};

use super::{
    app_disconnected_event::AppDisconnectedEvent,
    connect::{ConnectRequest, ConnectResponse},
    get_info::{GetInfoRequest, GetInfoResponse},
    get_pending_requests::{GetPendingRequestsRequest, GetPendingRequestsResponse},
    reject::Reject,
    sign_messages::{SignMessagesEvent, SignMessagesEventReply},
    sign_transation::{SignTransactionsEvent, SignTransactionsEventReply},
};

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize, TS)]
#[ts(export)]
#[serde(tag = "type")]
pub enum ClientToServer {
    ConnectRequest(ConnectRequest),
    GetInfoRequest(GetInfoRequest),
    SignTransactionsEventReply(SignTransactionsEventReply),
    SignMessagesEventReply(SignMessagesEventReply),
    GetPendingRequestsRequest(GetPendingRequestsRequest),
    Reject(Reject),
}
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize, TS)]
#[ts(export)]
#[serde(tag = "type")]
pub enum ServerToClient {
    GetInfoResponse(GetInfoResponse),
    ConnectResponse(ConnectResponse),
    SignTransactionsEvent(SignTransactionsEvent),
    SignMessagesEvent(SignMessagesEvent),
    AppDisconnectedEvent(AppDisconnectedEvent),
    GetPendingRequestsResponse(GetPendingRequestsResponse),
    ErrorMessage(ErrorMessage),
    AckMessage(AckMessage),
}
