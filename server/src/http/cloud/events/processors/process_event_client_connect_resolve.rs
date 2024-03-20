use crate::structs::cloud::cloud_events::event_types::client_connect_resolve_event::ClientConnectResolveEvent;
use database::{db::Db, tables::utils::get_current_datetime};
use log::error;
use std::{net::SocketAddr, sync::Arc};

pub async fn process_event_client_connect_resolve(
    event: &ClientConnectResolveEvent,
    app_id: &String,
    ip: SocketAddr,
    db: &Arc<Db>,
) {
    // // Check if connection attempt by client was successful, if not then there is nothing to do
    // if event.success {
    //     if let Err(err) = db
    //         .connect_user_to_the_session(
    //             &event.client_id,
    //             &event.wallet_name,
    //             &event.wallet_type,
    //             &get_current_datetime(),
    //             &event.public_keys,
    //             &app_id.clone(),
    //             &event.session_id,
    //         )
    //         .await
    //     {
    //         error!(
    //             "Failed to process user successful connect, app_id: [{}], ip: [{}], event: [{:?}], err: [{}]",
    //             app_id, ip, event, err
    //         );
    //     }
    // }
}
