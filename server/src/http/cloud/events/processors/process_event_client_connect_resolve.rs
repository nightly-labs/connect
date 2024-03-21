use crate::structs::cloud::cloud_events::event_types::client_connect_resolve_event::ClientConnectResolveEvent;
use database::{db::Db, tables::utils::get_current_datetime};
use log::error;
use std::{net::SocketAddr, sync::Arc};

pub async fn process_event_client_connect_resolve(
    event: &ClientConnectResolveEvent,
    app_id: &String,
    ip: SocketAddr,
    db: &Arc<Db>,
) {
    // Save event to Db
    save_event_client_connect_resolve(db, app_id, event).await;

    // Check if connection attempt by client was successful, if not then there is nothing to do
    if event.success {
        if let Err(err) = db
            .connect_user_to_the_session(
                &event.client_id,
                &event.wallet_name,
                &event.wallet_type,
                &get_current_datetime(),
                &event.addresses,
                &app_id.clone(),
                &event.session_id,
            )
            .await
        {
            error!(
                "Failed to process user successful connect, app_id: [{}], ip: [{}], event: [{:?}], err: [{}]",
                app_id, ip, event, err
            );
        }
    }
}

async fn save_event_client_connect_resolve(
    db: &Arc<Db>,
    app_id: &String,
    event: &ClientConnectResolveEvent,
) {
    // Establish a new transaction
    match db.connection_pool.begin().await {
        Ok(mut tx) => {
            // Update the connection status for the user
            match db
                .update_event_client_connect(
                    &mut tx,
                    &event.client_id,
                    &event.session_id,
                    event.success,
                    &event.addresses,
                )
                .await
            {
                Ok(_) => {
                    // Commit the transaction
                    if let Err(err) = tx.commit().await {
                        error!(
                            "Failed to commit transaction for update client connect event, app_id: [{}], event: [{:?}], err: [{}]",
                            app_id, event, err
                        );
                    }
                }
                Err(err) => {
                    error!(
                        "Failed to update client connect status, app_id: [{}], event: [{:?}], err: [{}]",
                        app_id, event, err
                    );

                    // Rollback the transaction
                    if let Err(err) = tx.rollback().await {
                        error!(
                            "Failed to rollback transaction for update client connect event, app_id: [{}], event: [{:?}], err: [{}]",
                            app_id, event, err
                        );

                        return;
                    }
                }
            }
        }
        Err(err) => {
            error!(
                "Failed to create new transaction to save client connect event, app_id: [{}], event: [{:?}], err: [{}]",
                app_id, event, err
            );
        }
    }
}
