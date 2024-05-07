use crate::structs::cloud::cloud_events::event_types::sign_transaction_event::SignTransactionEvent;
use database::{db::Db, structs::event_type::EventType, tables::utils::get_current_datetime};
use log::error;
use std::sync::Arc;

pub async fn process_event_sign_transaction(
    event: &SignTransactionEvent,
    app_id: &String,
    db: &Arc<Db>,
) {
    // Establish a new transaction
    let mut tx = match db.connection_pool.begin().await {
        Ok(tx) => tx,
        Err(err) => {
            error!(
                "Failed to create new transaction to save sign transaction event, app_id: [{}], event: [{:?}], err: [{}]",
                app_id, event, err
            );
            return;
        }
    };

    // Create a new event index
    let event_id = match db
        .create_new_event_entry(
            &mut tx,
            &app_id,
            &EventType::SignTransaction,
            &get_current_datetime(),
        )
        .await
    {
        Ok(event_id) => event_id,
        Err(err) => {
            error!(
                "Failed to create new event index, app_id: [{}], event: [{:?}], err: [{}]",
                app_id, event, err
            );

            // Rollback the transaction
            if let Err(err) = tx.rollback().await {
                error!(
                    "Failed to rollback transaction for new sign transaction event, app_id: [{}], event: [{:?}], err: [{}]",
                    app_id, event, err
                );
            }

            return;
        }
    };

    // Now create a new event sign transaction corresponding to the event
    match db
        .create_new_event_sign_transaction(
            &mut tx,
            event_id,
            &event.session_id,
            &event.request_id,
            &event.network,
        )
        .await
    {
        Ok(_) => {
            // Commit the transaction
            if let Err(err) = tx.commit().await {
                error!(
                    "Failed to commit transaction for new sign transaction event, app_id: [{}], event: [{:?}], err: [{}]",
                    app_id, event, err
                );
            }
        }
        Err(err) => {
            error!(
                "Failed to create new sign transaction event, app_id: [{}], event: [{:?}], err: [{}]",
                app_id, event, err
            );

            // Rollback the transaction
            if let Err(err) = tx.rollback().await {
                error!(
                    "Failed to rollback transaction for new sign transaction event, app_id: [{}], event: [{:?}], err: [{}]",
                    app_id, event, err
                );
            }
        }
    }
}
