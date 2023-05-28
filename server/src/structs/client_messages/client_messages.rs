use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::structs::common::ErrorMessage;

use super::{
    connect::{ConnectRequest, ConnectResponse},
    get_info::{GetInfoRequest, GetInfoResponse},
    get_pending_requests::{GetPendingRequestsRequest, GetPendingRequestsResponse},
    sign_transation::{SignTransactionsEvent, SignTransactionsEventReply},
};

#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize, TS)]
#[ts(export)]
#[serde(tag = "type")]
pub enum ClientToServer {
    ConnectRequest(ConnectRequest),
    GetInfoRequest(GetInfoRequest),
    SignTransactionsEventReply(SignTransactionsEventReply),
    GetPendingRequestsRequest(GetPendingRequestsRequest),
}
#[derive(Debug, Eq, PartialEq, Clone, Deserialize, Serialize, TS)]
#[ts(export)]
#[serde(tag = "type")]
pub enum ServerToClient {
    GetInfoResponse(GetInfoResponse),
    ConnectResponse(ConnectResponse),
    SignTransactionsEvent(SignTransactionsEvent),
    GetPendingRequestsResponse(GetPendingRequestsResponse),
    ErrorMessage(ErrorMessage),
}
