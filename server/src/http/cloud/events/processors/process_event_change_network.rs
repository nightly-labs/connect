use crate::structs::cloud::cloud_events::event_types::change_network_event::ChangeNetworkEvent;
use database::{db::Db, structs::event_type::EventType};
use log::error;
use std::sync::Arc;

pub async fn process_event_change_network(
    event: &ChangeNetworkEvent,
    app_id: &String,
    db: &Arc<Db>,
) {
    // Establish a new transaction
    match db.connection_pool.begin().await {
        Ok(mut tx) => {
            // Create a new event index in the database
            match db
                .create_new_event_entry(&mut tx, &app_id, &EventType::ChangeNetwork)
                .await
            {
                Ok(event_id) => {
                    // Now create a new event change network corresponding to the event
                    match db
                        .create_new_event_change_network(
                            &mut tx,
                            event_id,
                            &event.session_id,
                            &event.request_id,
                            &event.old_network,
                        )
                        .await
                    {
                        Ok(_) => {
                            // Commit the transaction
                            if let Err(err) = tx.commit().await {
                                error!(
                                    "Failed to commit transaction for new change network event, app_id: [{}], event: [{:?}], err: [{}]",
                                    app_id, event, err
                                );
                            }

                            return;
                        }
                        Err(err) => {
                            error!(
                                "Failed to create new change network event, app_id: [{}], event: [{:?}], err: [{}]",
                                app_id, event, err
                            );
                        }
                    }
                }
                Err(err) => {
                    error!(
                        "Failed to create new event index, app_id: [{}], event: [{:?}], err: [{}]",
                        app_id, event, err
                    );
                }
            }

            // If we have not returned yet, then we have failed to save the event
            // Rollback the transaction
            if let Err(err) = tx.rollback().await {
                error!(
                    "Failed to rollback transaction for new change network event, app_id: [{}], event: [{:?}], err: [{}]",
                    app_id, event, err
                );
            }
        }
        Err(err) => {
            error!(
                "Failed to create new transaction to save change network event, app_id: [{}], event: [{:?}], err: [{}]",
                app_id, event, err
            );
        }
    }
}
