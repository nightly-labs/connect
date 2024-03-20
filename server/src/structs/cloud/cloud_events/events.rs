use super::event_types::{
    app_connect_event::AppConnectEvent, app_disconnect_event::AppDisconnectEvent,
    client_connect_init_event::ClientConnectInitEvent,
    client_connect_resolve_event::ClientConnectResolveEvent,
    client_disconnect_event::ClientDisconnectEvent, new_request::NewRequestEvent,
    request_resolved_event::RequestResolvedEvent,
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(tag = "type")]
pub enum EventData {
    AppConnect(AppConnectEvent),
    AppDisconnect(AppDisconnectEvent),
    ClientConnectInit(ClientConnectInitEvent),
    ClientConnectResolve(ClientConnectResolveEvent),
    ClientDisconnect(ClientDisconnectEvent),
    NewRequest(NewRequestEvent),
    RequestResolved(RequestResolvedEvent),
}
