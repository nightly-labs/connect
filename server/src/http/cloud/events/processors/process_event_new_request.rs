use crate::structs::cloud::cloud_events::event_types::new_request::NewRequestEvent;
use database::{
    db::Db,
    structs::request_status::RequestStatus,
    tables::{requests::table_struct::Request, utils::get_current_datetime},
};
use log::error;
use std::{net::SocketAddr, sync::Arc};

pub async fn process_event_new_request(
    event: &NewRequestEvent,
    app_id: &String,
    ip: SocketAddr,
    db: &Arc<Db>,
) {
    // Save new request
    let request = Request {
        request_id: event.request_id.clone(),
        session_id: event.session_id.clone(),
        app_id: app_id.clone(),
        request_type: event.request_type.clone(),
        request_status: RequestStatus::Pending,
        network: event.network.clone(),
        creation_timestamp: get_current_datetime(),
    };

    if let Err(err) = db.save_request(&request).await {
        error!(
            "Failed to save new request, app_id: [{}], ip: [{}], event: [{:?}], err: [{}]",
            app_id, ip, event, err
        );
    }
}
