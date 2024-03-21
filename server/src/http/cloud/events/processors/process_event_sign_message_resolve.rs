use crate::structs::cloud::cloud_events::event_types::sign_message_resolve_event::SignMessageResolveEvent;
use database::{db::Db, structs::request_status::RequestStatus};
use log::error;
use std::sync::Arc;

pub async fn process_event_sign_message_resolve(
    event: &SignMessageResolveEvent,
    app_id: &String,
    db: &Arc<Db>,
) {
    // Establish a new transaction
    let mut tx = match db.connection_pool.begin().await {
        Ok(tx) => tx,
        Err(err) => {
            error!(
            "Failed to create new transaction to save sign message event, app_id: [{}], event: [{:?}], err: [{}]",
            app_id, event, err
        );
            return;
        }
    };

    // Update the sign message event in the database
    match db
        .update_event_sign_message(
            &mut tx,
            &event.request_id,
            RequestStatus::from(&event.failure_reason),
        )
        .await
    {
        Ok(_) => {
            // Commit the transaction
            if let Err(err) = tx.commit().await {
                error!(
                    "Failed to commit transaction for update sign message event, app_id: [{}], event: [{:?}], err: [{}]",
                    app_id, event, err
                );
            }
        }
        Err(err) => {
            error!(
                "Failed to update sign message status, app_id: [{}], event: [{:?}], err: [{}]",
                app_id, event, err
            );

            // Rollback the transaction
            if let Err(err) = tx.rollback().await {
                error!(
                    "Failed to rollback transaction for update sign message event, app_id: [{}], event: [{:?}], err: [{}]",
                    app_id, event, err
                );
            }
        }
    }
}
