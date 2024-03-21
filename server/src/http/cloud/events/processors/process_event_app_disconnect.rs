use crate::structs::cloud::cloud_events::event_types::app_disconnect_event::AppDisconnectEvent;
use database::{db::Db, structs::event_type::EventType};
use log::error;
use std::{net::SocketAddr, sync::Arc};

pub async fn process_event_app_disconnect(
    event: &AppDisconnectEvent,
    app_id: &String,
    ip: SocketAddr,
    db: &Arc<Db>,
) {
    // Save event to Db
    save_event_app_disconnect(db, app_id, event).await;

    // Close app disconnect in the database
    if let Err(err) = db.close_app_connection(&event.session_id, &app_id).await {
        error!(
            "Failed to close app disconnect, app_id: [{}], ip: [{}], event: [{:?}], err: [{}]",
            app_id, ip, event, err
        );
    }
}

async fn save_event_app_disconnect(db: &Arc<Db>, app_id: &String, event: &AppDisconnectEvent) {
    // Establish a new transaction
    let mut tx = match db.connection_pool.begin().await {
        Ok(tx) => tx,
        Err(err) => {
            error!(
                "Failed to create new transaction to save app disconnect event, app_id: [{}], event: [{:?}], err: [{}]",
                app_id, event, err
            );
            return;
        }
    };

    // Create a new event index
    let event_id = match db
        .create_new_event_entry(&mut tx, &app_id, &EventType::AppDisconnect)
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
                    "Failed to rollback transaction for new app disconnect event, app_id: [{}], event: [{:?}], err: [{}]",
                    app_id, event, err
                );
            }

            return;
        }
    };

    // Now create a new event app disconnect corresponding to the event
    match db
        .create_new_event_app_disconnect(&mut tx, event_id, &event.session_id)
        .await
    {
        Ok(_) => {
            // Commit the transaction
            if let Err(err) = tx.commit().await {
                error!(
                    "Failed to commit transaction for new app disconnect event, app_id: [{}], event: [{:?}], err: [{}]",
                    app_id, event, err
                );
            }
        }
        Err(err) => {
            error!(
                "Failed to create new app disconnect event, app_id: [{}], event: [{:?}], err: [{}]",
                app_id, event, err
            );

            // Rollback the transaction
            if let Err(err) = tx.rollback().await {
                error!(
                    "Failed to rollback transaction for new app disconnect event, app_id: [{}], event: [{:?}], err: [{}]",
                    app_id, event, err
                );
            }
        }
    }
}
