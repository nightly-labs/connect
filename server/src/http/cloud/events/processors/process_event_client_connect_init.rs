use crate::{
    ip_geolocation::GeolocationRequester,
    structs::cloud::cloud_events::event_types::client_connect_init_event::ClientConnectInitEvent,
    utils::get_geolocation_data,
};
use database::db::Db;
use log::error;
use std::{net::SocketAddr, sync::Arc};

pub async fn process_event_client_connect_init(
    event: &ClientConnectInitEvent,
    app_id: &String,
    ip: SocketAddr,
    db: &Arc<Db>,
    geo_loc_requester: &Arc<GeolocationRequester>,
) {
    // Save connection attempt by client
    let mut tx = db.connection_pool.begin().await.unwrap();

    // Get the geolocation data
    let geo_location_data = get_geolocation_data(&db, &geo_loc_requester, &ip).await;

    match db
        .create_new_connection_event_by_client(
            &mut tx,
            &app_id,
            &event.session_id,
            &event.session_type,
            &ip.to_string(),
            geo_location_data,
        )
        .await
    {
        Ok(_) => {
            // Commit the transaction
            if let Err(err) = tx.commit().await {
                error!(
                    "Failed to commit transaction for new app connection event, app_id: [{}], ip: [{}], event: [{:?}], err: [{}]",
                    app_id, ip, event, err
                );
            }
        }
        Err(err) => {
            error!(
                "Failed to create new connection event, app_id: [{}], ip: [{}], event: [{:?}], err: [{}]",
                app_id, ip, event, err
            );
        }
    }
}
