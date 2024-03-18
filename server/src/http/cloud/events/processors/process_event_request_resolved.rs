use crate::structs::cloud::cloud_events::event_types::request_resolved_event::RequestResolvedEvent;
use database::db::Db;
use std::{net::SocketAddr, sync::Arc};

pub async fn process_event_request_resolved(
    event: &RequestResolvedEvent,
    app_id: &String,
    ip: SocketAddr,
    db: &Arc<Db>,
) {
    // TODO Update request status in database
}
