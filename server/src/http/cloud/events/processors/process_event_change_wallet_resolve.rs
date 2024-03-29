use crate::structs::cloud::cloud_events::event_types::change_wallet_resolve_event::ChangeWalletResolveEvent;
use database::{db::Db, structs::request_status::RequestStatus};
use log::error;
use std::sync::Arc;

pub async fn process_event_change_wallet_resolve(
    event: &ChangeWalletResolveEvent,
    app_id: &String,
    db: &Arc<Db>,
) {
    // Establish a new transaction
    let mut tx = match db.connection_pool.begin().await {
        Ok(tx) => tx,
        Err(err) => {
            error!(
                "Failed to create new transaction to save change wallet event, app_id: [{}], event: [{:?}], err: [{}]",
                app_id, event, err
            );
            return;
        }
    };

    // Update the change wallet event in the database
    match db
        .update_event_change_wallet(
            &mut tx,
            &event.request_id,
            RequestStatus::from(&event.failure_reason),
            &event.new_wallet_address,
        )
        .await
    {
        Ok(_) => {
            // Commit the transaction
            if let Err(err) = tx.commit().await {
                error!(
                    "Failed to commit transaction for update change wallet event, app_id: [{}], event: [{:?}], err: [{}]",
                    app_id, event, err
                );
            }
        }
        Err(err) => {
            error!(
                "Failed to update change wallet status, app_id: [{}], event: [{:?}], err: [{}]",
                app_id, event, err
            );

            // Rollback the transaction
            if let Err(err) = tx.rollback().await {
                error!(
                    "Failed to rollback transaction for update change wallet event, app_id: [{}], event: [{:?}], err: [{}]",
                    app_id, event, err
                );
            }
        }
    }
}
