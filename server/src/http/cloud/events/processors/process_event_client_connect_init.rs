use crate::structs::cloud::cloud_events::event_types::client_connect_init_event::ClientConnectInitEvent;
use database::db::Db;
use std::{net::SocketAddr, sync::Arc};

pub async fn process_event_client_connect_init(
    event: &ClientConnectInitEvent,
    app_id: &String,
    ip: SocketAddr,
    db: &Arc<Db>,
) {
    // TODO Save connection attempt by client
}
