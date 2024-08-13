use crate::structs::cloud::cloud_events::event_types::change_wallet_event::ChangeWalletEvent;
use database::{db::Db, structs::event_type::EventType, tables::utils::get_current_datetime};
use log::error;
use std::sync::Arc;

pub async fn process_event_change_wallet(
    event: &ChangeWalletEvent,
    app_id: &String,
    network: &String,
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

    // Create a new event index
    let event_id = match db
        .create_new_event_entry(
            &mut tx,
            &app_id,
            &network,
            &&EventType::ChangeWallet,
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
                    "Failed to rollback transaction for new change wallet event, app_id: [{}], event: [{:?}], err: [{}]",
                    app_id, event, err
                );
            }

            return;
        }
    };

    // Now create a new event change wallet corresponding to the event
    match db
        .create_new_event_change_wallet(
            &mut tx,
            event_id,
            &event.session_id,
            &event.request_id,
            &event.network,
            &event.wallet_name,
            &event.wallet_type,
            &event.old_wallet_address,
        )
        .await
    {
        Ok(_) => {
            // Commit the transaction
            if let Err(err) = tx.commit().await {
                error!(
                    "Failed to commit transaction for new change wallet event, app_id: [{}], event: [{:?}], err: [{}]",
                    app_id, event, err
                );
            }
        }
        Err(err) => {
            error!(
                "Failed to create new change wallet event, app_id: [{}], event: [{:?}], err: [{}]",
                app_id, event, err
            );

            // Rollback the transaction
            if let Err(err) = tx.rollback().await {
                error!(
                    "Failed to rollback transaction for new change wallet event, app_id: [{}], event: [{:?}], err: [{}]",
                    app_id, event, err
                );
            }
        }
    }
}
