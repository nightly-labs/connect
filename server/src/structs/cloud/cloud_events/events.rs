use super::event_types::{
    app_connect_event::AppConnectEvent, app_disconnect_event::AppDisconnectEvent,
    client_connect_event::ClientConnectEvent, client_disconnect_event::ClientDisconnectEvent,
    new_request::NewRequestEvent, request_resolved_event::RequestResolvedEvent,
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum EventData {
    AppConnect(AppConnectEvent),
    AppDisconnect(AppDisconnectEvent),
    ClientConnect(ClientConnectEvent),
    ClientDisconnect(ClientDisconnectEvent),
    NewRequest(NewRequestEvent),
    RequestResolved(RequestResolvedEvent),
}
