use super::{
    events::HttpNightlyConnectCloudEvent,
    processors::{
        process_event_app_connect::process_event_app_connect,
        process_event_app_disconnect::process_event_app_disconnect,
        process_event_change_network::process_event_change_network,
        process_event_change_wallet::process_event_change_wallet,
        process_event_client_connect::process_event_client_connect_init,
        process_event_client_connect_resolve::process_event_client_connect_resolve,
        process_event_client_disconnect::process_event_client_disconnect,
        process_event_sign_and_send_transaction::process_event_sign_and_send_transaction,
        process_event_sign_and_send_transaction_resolve::process_event_sign_and_send_transaction_resolve,
        process_event_sign_message::process_event_sign_message,
        process_event_sign_message_resolve::process_event_sign_message_resolve,
        process_event_sign_transaction::process_event_sign_transaction,
        process_event_sign_transaction_resolve::process_event_sign_transaction_resolve,
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
        EventData::ClientConnect(event) => {
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
        EventData::SignMessage(event) => {
            process_event_sign_message(event, &event_payload.app_id, db_connection).await;
        }
        EventData::SignMessageResolve(event) => {
            process_event_sign_message_resolve(event, &event_payload.app_id, db_connection).await;
        }
        EventData::SignTransaction(event) => {
            process_event_sign_transaction(event, &event_payload.app_id, db_connection).await;
        }
        EventData::SignTransactionResolve(event) => {
            process_event_sign_transaction_resolve(event, &event_payload.app_id, db_connection)
                .await;
        }
        EventData::SignAndSendTransaction(event) => {
            process_event_sign_and_send_transaction(event, &event_payload.app_id, db_connection)
                .await;
        }
        EventData::SignAndSendTransactionResolve(event) => {
            process_event_sign_and_send_transaction_resolve(
                event,
                &event_payload.app_id,
                db_connection,
            )
            .await;
        }
        EventData::ChangeNetwork(event) => {
            process_event_change_network(event, &event_payload.app_id, db_connection).await;
        }
        EventData::ChangeWallet(event) => {
            process_event_change_wallet(event, &event_payload.app_id, db_connection).await;
        }
    }
}
