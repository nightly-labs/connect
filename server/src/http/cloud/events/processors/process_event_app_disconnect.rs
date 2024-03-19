use crate::structs::cloud::cloud_events::event_types::app_disconnect_event::AppDisconnectEvent;
use database::db::Db;
use log::{error, warn};
use std::{net::SocketAddr, sync::Arc};

pub async fn process_event_app_disconnect(
    event: &AppDisconnectEvent,
    app_id: &String,
    ip: SocketAddr,
    db: &Arc<Db>,
) {
    // Close app connection in the database
    let mut tx = db.connection_pool.begin().await.unwrap();
    match db
        .close_app_connection(&mut tx, &event.session_id, &app_id)
        .await
    {
        Ok(_) => {
            // Commit the transaction
            if let Err(err) = tx.commit().await {
                error!(
                        "Failed to commit transaction for close app connection, app_id: [{}], ip: [{}], event: [{:?}], err: [{}]",
                        app_id, ip, event, err
                    );
            }
        }
        Err(err) => {
            error!(
                "Failed to close app connection, app_id: [{}], ip: [{}], event: [{:?}], err: [{}]",
                app_id, ip, event, err
            );

            // Rollback the transaction
            if let Err(err) = tx.rollback().await {
                warn!(
                    "Failed to rollback transaction for new ip address, ip: [{}], err: [{}]",
                    ip, err
                );
            }
        }
    }
}