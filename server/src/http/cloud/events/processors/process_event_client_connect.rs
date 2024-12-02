use crate::{
    http::cloud::utils::get_geolocation_data, ip_geolocation::GeolocationRequester,
    structs::cloud::cloud_events::event_types::client_connect_event::ClientConnectEvent,
};
use chrono::{DateTime, Utc};
use database::{db::Db, structs::event_type::EventType, tables::utils::get_current_datetime};
use log::error;
use std::{net::SocketAddr, sync::Arc};

pub async fn process_event_client_connect_init(
    event: &ClientConnectEvent,
    app_id: &String,
    network: &String,
    ip: SocketAddr,
    db: &Arc<Db>,
    geo_loc_requester: &Arc<GeolocationRequester>,
) {
    let current_time = get_current_datetime();

    // Save event to Db
    save_event_client_connect(db, app_id, network, event, &current_time).await;

    // Save connection attempt by client
    let mut tx = match db.connection_pool.begin().await {
        Ok(tx) => tx,
        Err(err) => {
            error!(
                "Failed to create new transaction to save client connect event, app_id: [{}], event: [{:?}], err: [{}]",
                app_id, event, err
            );
            return;
        }
    };

    // Get the geolocation data
    let geo_location_data = get_geolocation_data(&db, &geo_loc_requester, &ip).await;

    match db
        .create_new_connection_event_by_client(
            &mut tx,
            &app_id,
            &network,
            &event.session_id,
            &event.session_type,
            &ip.to_string(),
            geo_location_data,
            &current_time,
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

async fn save_event_client_connect(
    db: &Arc<Db>,
    app_id: &String,
    network: &String,
    event: &ClientConnectEvent,
    current_time: &DateTime<Utc>,
) {
    // Establish a new transaction
    let mut tx = match db.connection_pool.begin().await {
        Ok(tx) => tx,
        Err(err) => {
            error!(
                "Failed to create new transaction to save client connect event, app_id: [{}], event: [{:?}], err: [{}]",
                app_id, event, err
            );
            return;
        }
    };

    // Create a new event index
    let event_id = match db
        .create_new_event_entry(
            &mut tx,
            &app_id,
            network,
            &EventType::ClientConnect,
            &current_time,
        )
        .await
    {
        Ok(event_id) => event_id,
        Err(err) => {
            error!(
                "Failed to create new event index, app_id: [{}], event: [{:?}], err: [{}]",
                app_id, event, err
            );

            // Rollback the transaction
            if let Err(err) = tx.rollback().await {
                error!(
                    "Failed to rollback transaction for new client connect event, app_id: [{}], event: [{:?}], err: [{}]",
                    app_id, event, err
                );
            }

            return;
        }
    };

    // Now create a event client connect corresponding to the event
    match db
        .create_new_event_client_connect(
            &mut tx,
            event_id,
            &event.client_id,
            &event.session_id,
            &event.wallet_name,
            &event.wallet_type,
            &event.session_type,
        )
        .await
    {
        Ok(_) => {
            // Commit the transaction
            if let Err(err) = tx.commit().await {
                error!(
                    "Failed to commit transaction for new client connect event, app_id: [{}], event: [{:?}], err: [{}]",
                    app_id, event, err
                );
            }
        }
        Err(err) => {
            error!(
                "Failed to create new client connect event, app_id: [{}], event: [{:?}], err: [{}]",
                app_id, event, err
            );

            // Rollback the transaction
            if let Err(err) = tx.rollback().await {
                error!(
                    "Failed to rollback transaction for new client connect event, app_id: [{}], event: [{:?}], err: [{}]",
                    app_id, event, err
                );
            }
        }
    }
}
