use crate::structs::cloud::cloud_events::event_types::sign_message_event::SignMessageEvent;
use database::{db::Db, structs::event_type::EventType};
use log::error;
use std::sync::Arc;

pub async fn process_event_sign_message(event: &SignMessageEvent, app_id: &String, db: &Arc<Db>) {
    // Establish a new transaction
    match db.connection_pool.begin().await {
        Ok(mut tx) => {
            // Create a new event index in the database
            match db
                .create_new_event_entry(&mut tx, &app_id, &EventType::SignMessage)
                .await
            {
                Ok(event_id) => {
                    // Now create a new event sign message corresponding to the event
                    match db
                        .create_new_event_sign_message(
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
                                    "Failed to commit transaction for new sign message event, app_id: [{}], event: [{:?}], err: [{}]",
                                    app_id, event, err
                                );
                            }

                            return;
                        }
                        Err(err) => {
                            error!(
                                "Failed to create new sign message event, app_id: [{}], event: [{:?}], err: [{}]",
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
                    "Failed to rollback transaction for new sign message event, app_id: [{}], event: [{:?}], err: [{}]",
                    app_id, event, err
                );
            }
        }
        Err(err) => {
            error!(
                "Failed to create new transaction to save sign message event, app_id: [{}], event: [{:?}], err: [{}]",
                app_id, event, err
            );
        }
    }
}
