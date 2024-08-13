use crate::structs::cloud::cloud_events::event_types::client_disconnect_event::ClientDisconnectEvent;
use chrono::{DateTime, Utc};
use database::{db::Db, structs::event_type::EventType, tables::utils::get_current_datetime};
use log::error;
use std::{net::SocketAddr, sync::Arc};

pub async fn process_event_client_disconnect(
    event: &ClientDisconnectEvent,
    app_id: &String,
    network: &String,
    ip: SocketAddr,
    db: &Arc<Db>,
) {
    let current_timestamp = get_current_datetime();

    // Save event to Db
    save_event_client_disconnect(db, app_id, network, event, &current_timestamp).await;

    // Update connection status for user
    let mut tx = db.connection_pool.begin().await.unwrap();

    match db
        .close_client_connection(&mut tx, &app_id, &event.disconnected_session_id)
        .await
    {
        Ok(_) => {
            // Commit the transaction
            if let Err(err) = tx.commit().await {
                error!(
                     "Failed to commit transaction for client disconnect event, app_id: [{}], ip: [{}], event: [{:?}], err: [{}]",
                     app_id, ip, event, err
                 );
            }
        }
        Err(err) => {
            error!(
                 "Failed to update client disconnect status, app_id: [{}], ip: [{}], event: [{:?}], err: [{}]",
                 app_id, ip, event, err
             );
        }
    }
}

async fn save_event_client_disconnect(
    db: &Arc<Db>,
    app_id: &String,
    network: &String,
    event: &ClientDisconnectEvent,
    current_timestamp: &DateTime<Utc>,
) {
    // Establish a new transaction
    let mut tx = match db.connection_pool.begin().await {
        Ok(tx) => tx,
        Err(err) => {
            error!(
                "Failed to create new transaction to save client disconnect event, app_id: [{}], event: [{:?}], err: [{}]",
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
            &network,
            &EventType::ClientDisconnect,
            current_timestamp,
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
                    "Failed to rollback transaction for new client disconnect event, app_id: [{}], event: [{:?}], err: [{}]",
                    app_id, event, err
                );
            }

            return;
        }
    };

    // Now create a event client disconnect corresponding to the event
    match db
        .create_new_event_client_disconnect(
            &mut tx,
            event_id,
            &event.client_id,
            &event.disconnected_session_id,
        )
        .await
    {
        Ok(_) => {
            // Commit the transaction
            if let Err(err) = tx.commit().await {
                error!(
                    "Failed to commit transaction for new client disconnect event, app_id: [{}], event: [{:?}], err: [{}]",
                    app_id, event, err
                );
            }
        }
        Err(err) => {
            error!(
                "Failed to create new client disconnect event, app_id: [{}], event: [{:?}], err: [{}]",
                app_id, event, err
            );

            // Rollback the transaction
            if let Err(err) = tx.rollback().await {
                error!(
                    "Failed to rollback transaction for new client disconnect event, app_id: [{}], event: [{:?}], err: [{}]",
                    app_id, event, err
                );
            }
        }
    }
}
