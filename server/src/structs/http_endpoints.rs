use serde::{Deserialize, Serialize};
use ts_rs::TS;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum HttpEndpoint {
    #[serde(rename = "/get_sessions")]
    GetSessions,
    #[serde(rename = "/get_session_info")]
    GetSessionInfo,
    #[serde(rename = "/connect_session")]
    ConnectSession,
    #[serde(rename = "/drop_sessions")]
    DropSessions,
    #[serde(rename = "/resolve_request")]
    ResolveRequest,
    #[serde(rename = "/get_pending_requests")]
    GetPendingRequests,
    #[serde(rename = "/get_pending_request")]
    GetPendingRequest,
    #[serde(rename = "/get_wallets_metadata")]
    GetWalletsMetadata,
}

impl HttpEndpoint {
    pub fn to_string(&self) -> String {
        match self {
            HttpEndpoint::GetSessions => "/get_sessions".to_string(),
            HttpEndpoint::GetSessionInfo => "/get_session_info".to_string(),
            HttpEndpoint::ConnectSession => "/connect_session".to_string(),
            HttpEndpoint::DropSessions => "/drop_sessions".to_string(),
            HttpEndpoint::ResolveRequest => "/resolve_request".to_string(),
            HttpEndpoint::GetPendingRequests => "/get_pending_requests".to_string(),
            HttpEndpoint::GetPendingRequest => "/get_pending_request".to_string(),
            HttpEndpoint::GetWalletsMetadata => "/get_wallets_metadata".to_string(),
        }
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_to_string() {
        let endpoint = super::HttpEndpoint::GetSessions;
        assert_eq!(endpoint.to_string(), "/get_sessions");
    }
}
