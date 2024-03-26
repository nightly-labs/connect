use crate::structs::cloud::cloud_events::event_types::sign_and_send_transaction_resolve_event::SignAndSendTransactionResolveEvent;
use database::{db::Db, structs::request_status::RequestStatus};
use log::error;
use std::sync::Arc;

pub async fn process_event_sign_and_send_transaction_resolve(
    event: &SignAndSendTransactionResolveEvent,
    app_id: &String,
    db: &Arc<Db>,
) {
    // Establish a new transaction
    let mut tx = match db.connection_pool.begin().await {
        Ok(tx) => tx,
        Err(err) => {
            error!(
                "Failed to create new transaction to save sign and send transaction event, app_id: [{}], event: [{:?}], err: [{}]",
                app_id, event, err
            );
            return;
        }
    };

    // Update the sign and send transaction event in the database
    match db
        .update_event_sign_and_send_transaction(
            &mut tx,
            &event.request_id,
            RequestStatus::from(&event.failure_reason),
            &event.tx_hash,
        )
        .await
    {
        Ok(_) => {
            // Commit the transaction
            if let Err(err) = tx.commit().await {
                error!(
                    "Failed to commit transaction for update sign and send transaction event, app_id: [{}], event: [{:?}], err: [{}]",
                    app_id, event, err
                );
            }
        }
        Err(err) => {
            error!(
                "Failed to update sign and send transaction status, app_id: [{}], event: [{:?}], err: [{}]",
                app_id, event, err
            );

            // Rollback the transaction
            if let Err(err) = tx.rollback().await {
                error!(
                    "Failed to rollback transaction for update sign and send transaction event, app_id: [{}], event: [{:?}], err: [{}]",
                    app_id, event, err
                );
            }
        }
    }
}
