use crate::structs::cloud::cloud_events::event_types::new_request::NewRequestEvent;
use database::db::Db;
use std::{net::SocketAddr, sync::Arc};

pub async fn process_event_new_request(
    event: &NewRequestEvent,
    app_id: &String,
    ip: SocketAddr,
    db: &Arc<Db>,
) {
    // TODO Save new request
}
