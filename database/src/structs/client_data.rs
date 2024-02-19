#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClientData {
    pub client_id: Option<String>,
    pub device: Option<String>,
    pub metadata: Option<String>,
    pub notification_endpoint: Option<String>,
    pub connected_at: u64, // Timestamp of when the client connected to the session
}
