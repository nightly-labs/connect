use super::{
    events::HttpNightlyConnectCloudEvent,
    processors::{
        process_event_app_connect::process_event_app_connect,
        process_event_app_disconnect::process_event_app_disconnect,
    },
};
use crate::{state::Sessions, structs::cloud::cloud_events::events::EventData};
use database::db::Db;
use std::{net::SocketAddr, sync::Arc};

pub async fn process_event(
    event_payload: HttpNightlyConnectCloudEvent,
    ip: SocketAddr,
    db_connection: &Arc<Db>,
    sessions: &Sessions,
) {
    match &event_payload.event {
        EventData::AppConnect(event) => {
            process_event_app_connect(event, &event_payload.app_id, ip, db_connection, sessions)
                .await;
        }
        EventData::AppDisconnect(event) => {
            process_event_app_disconnect(event, &event_payload.app_id, ip, db_connection).await;
        }
        EventData::ClientConnectInit(event) => todo!(),
        EventData::ClientConnectResolve(event) => todo!(),
        EventData::ClientDisconnect(event) => todo!(),
        EventData::NewRequest(event) => todo!(),
        EventData::RequestResolved(event) => todo!(),
    }
}
