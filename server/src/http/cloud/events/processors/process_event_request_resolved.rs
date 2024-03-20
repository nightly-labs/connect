use crate::structs::cloud::cloud_events::event_types::request_resolved_event::RequestResolvedEvent;
use database::{db::Db, structs::request_status::RequestStatus};
use log::error;
use std::{net::SocketAddr, sync::Arc};

pub async fn process_event_request_resolved(
    event: &RequestResolvedEvent,
    app_id: &String,
    ip: SocketAddr,
    db: &Arc<Db>,
) {
    // Generate new request status based on event data
    let new_status: RequestStatus = match event.failure_reason.clone() {
        Some(reason) => reason.into(),
        None => RequestStatus::Completed,
    };

    // Update request status in database
    if let Err(err) = db
        .update_request_status(&event.request_id, &new_status)
        .await
    {
        error!(
            "Failed to save new request, app_id: [{}], ip: [{}], event: [{:?}], err: [{}]",
            app_id, ip, event, err
        );
    }
}
