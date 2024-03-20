use crate::structs::cloud::cloud_events::event_types::client_disconnect_event::ClientDisconnectEvent;
use database::db::Db;
use log::error;
use std::{net::SocketAddr, sync::Arc};

pub async fn process_event_client_disconnect(
    event: &ClientDisconnectEvent,
    app_id: &String,
    ip: SocketAddr,
    db: &Arc<Db>,
) {
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
