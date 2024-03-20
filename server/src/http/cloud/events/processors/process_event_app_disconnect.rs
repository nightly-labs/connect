use crate::structs::cloud::cloud_events::event_types::app_disconnect_event::AppDisconnectEvent;
use database::db::Db;
use log::error;
use std::{net::SocketAddr, sync::Arc};

pub async fn process_event_app_disconnect(
    event: &AppDisconnectEvent,
    app_id: &String,
    ip: SocketAddr,
    db: &Arc<Db>,
) {
    // Close app connection in the database
    if let Err(err) = db.close_app_connection(&event.session_id, &app_id).await {
        error!(
            "Failed to close app connection, app_id: [{}], ip: [{}], event: [{:?}], err: [{}]",
            app_id, ip, event, err
        );
    }
}
