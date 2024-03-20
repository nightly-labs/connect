use super::{
    events::HttpNightlyConnectCloudEvent,
    processors::{
        process_event_app_connect::process_event_app_connect,
        process_event_app_disconnect::process_event_app_disconnect,
        process_event_client_connect_init::process_event_client_connect_init,
        process_event_client_connect_resolve::process_event_client_connect_resolve,
        process_event_client_disconnect::process_event_client_disconnect,
        process_event_new_request::process_event_new_request,
        process_event_request_resolved::process_event_request_resolved,
    },
};
use crate::{
    ip_geolocation::GeolocationRequester, state::Sessions,
    structs::cloud::cloud_events::events::EventData,
};
use database::db::Db;
use std::{net::SocketAddr, sync::Arc};

pub async fn process_event(
    event_payload: HttpNightlyConnectCloudEvent,
    ip: SocketAddr,
    db_connection: &Arc<Db>,
    geo_loc_requester: &Arc<GeolocationRequester>,
    sessions: &Sessions,
) {
    match &event_payload.event {
        EventData::AppConnect(event) => {
            process_event_app_connect(
                event,
                &event_payload.app_id,
                ip,
                db_connection,
                geo_loc_requester,
                sessions,
            )
            .await;
        }
        EventData::AppDisconnect(event) => {
            process_event_app_disconnect(event, &event_payload.app_id, ip, db_connection).await;
        }
        EventData::ClientConnectInit(event) => {
            process_event_client_connect_init(
                event,
                &event_payload.app_id,
                ip,
                db_connection,
                geo_loc_requester,
            )
            .await;
        }
        EventData::ClientConnectResolve(event) => {
            process_event_client_connect_resolve(event, &event_payload.app_id, ip, db_connection)
                .await;
        }
        EventData::ClientDisconnect(event) => {
            process_event_client_disconnect(event, &event_payload.app_id, ip, db_connection).await;
        }
        EventData::NewRequest(event) => {
            process_event_new_request(event, &event_payload.app_id, ip, db_connection).await;
        }
        EventData::RequestResolved(event) => {
            process_event_request_resolved(event, &event_payload.app_id, ip, db_connection).await;
        }
    }
}
