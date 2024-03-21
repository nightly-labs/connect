use crate::structs::cloud::cloud_events::event_types::change_network_resolve_event::ChangeNetworkResolveEvent;
use database::{db::Db, structs::request_status::RequestStatus};
use log::error;
use std::sync::Arc;

pub async fn process_event_change_network_resolve(
    event: &ChangeNetworkResolveEvent,
    app_id: &String,
    db: &Arc<Db>,
) {
    // Establish a new transaction
    match db.connection_pool.begin().await {
        Ok(mut tx) => {
            // Update the change network event in the database
            match db
                .update_event_change_network(
                    &mut tx,
                    &event.request_id,
                    RequestStatus::from(&event.failure_reason),
                    &event.new_network,
                )
                .await
            {
                Ok(_) => {
                    // Commit the transaction
                    if let Err(err) = tx.commit().await {
                        error!(
                                "Failed to commit transaction for update change network event, app_id: [{}], event: [{:?}], err: [{}]",
                                app_id, event, err
                            );
                    }
                }
                Err(err) => {
                    error!(
                            "Failed to update change network status, app_id: [{}], event: [{:?}], err: [{}]",
                            app_id, event, err
                        );

                    // Rollback the transaction
                    if let Err(err) = tx.rollback().await {
                        error!(
                                "Failed to rollback transaction for update change network event, app_id: [{}], event: [{:?}], err: [{}]",
                                app_id, event, err
                            );

                        return;
                    }
                }
            }
        }
        Err(err) => {
            error!(
                    "Failed to create new transaction to update change network event, app_id: [{}], event: [{:?}], err: [{}]",
                    app_id, event, err
                );
        }
    }
}
