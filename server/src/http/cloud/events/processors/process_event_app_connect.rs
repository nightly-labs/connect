use crate::{
    http::cloud::utils::get_geolocation_data, ip_geolocation::GeolocationRequester,
    state::Sessions, structs::cloud::cloud_events::event_types::app_connect_event::AppConnectEvent,
};
use chrono::{DateTime, Utc};
use database::{
    db::Db,
    structs::event_type::EventType,
    tables::{
        sessions::table_struct::DbNcSession,
        utils::{get_current_datetime, get_date_time},
    },
};
use log::error;
use std::{net::SocketAddr, sync::Arc};

pub async fn process_event_app_connect(
    event: &AppConnectEvent,
    app_id: &String,
    ip: SocketAddr,
    db: &Arc<Db>,
    geo_loc_requester: &Arc<GeolocationRequester>,
    sessions: &Sessions,
) {
    let current_timestamp = get_current_datetime();

    // Save event to Db
    save_event_app_connect(db, app_id, event, &current_timestamp).await;

    if event.new_session {
        // New session, get the data from sessions and create a new session in the database
        let session_data = {
            let sessions_read = sessions.read().await;
            let app_sessions_read = match sessions_read.get(app_id) {
                Some(session) => session.read().await,
                None => {
                    // We have just connected so it should exist
                    error!("App: [{}] does not have any sessions", app_id);
                    return;
                }
            };

            match app_sessions_read.get(&event.session_id) {
                Some(session) => {
                    let session = session.read().await;

                    DbNcSession {
                        session_id: session.session_id.clone(),
                        app_id: app_id.clone(),
                        // TODO update the app_metadata field in db
                        app_metadata: "App metadata".to_string(),
                        persistent: session.persistent,
                        network: session.network.0.clone(),
                        // Client data should be empty if the session was just created
                        client_data: None,
                        // Getting the current datetime from timestamp should never fail, just in case we will use the current datetime
                        session_open_timestamp: get_date_time(session.creation_timestamp)
                            .unwrap_or(get_current_datetime()),
                        session_close_timestamp: None,
                    }
                }
                None => {
                    // We have just connected so it should exist
                    error!(
                        "App: [{}] does not have session with id: [{}]",
                        app_id, event.session_id
                    );
                    return;
                }
            }
        };

        // Get the geolocation data
        let geo_location_data = get_geolocation_data(&db, &geo_loc_requester, &ip).await;

        // Should not fail, but if it does then we will have a problem
        if let Err(err) = db
            .handle_new_session(
                &session_data,
                geo_location_data,
                &ip.to_string(),
                &current_timestamp,
            )
            .await
        {
            error!(
                "Failed to create new session, app_id: [{}], ip: [{}], event: [{:?}], err: [{}]",
                app_id, ip, event, err
            );
        }
    } else {
        // Reconnection to an existing session
        let mut tx = db.connection_pool.begin().await.unwrap();

        // Get the geolocation data
        let geo_location_data = get_geolocation_data(&db, &geo_loc_requester, &ip).await;

        match db
            .create_new_connection_event_by_app(
                &mut tx,
                &event.session_id,
                &app_id,
                &ip.to_string(),
                geo_location_data,
                &current_timestamp,
            )
            .await
        {
            Ok(_) => {
                // Commit the transaction
                if let Err(err) = tx.commit().await {
                    error!(
                        "Failed to commit transaction for new app connection, app_id: [{}], ip: [{}], event: [{:?}], err: [{}]",
                        app_id, ip, event, err
                    );
                }
            }
            Err(err) => {
                error!(
                    "Failed to create new connection, app_id: [{}], ip: [{}], event: [{:?}], err: [{}]",
                    app_id, ip, event, err
                );
            }
        }
    }
}

async fn save_event_app_connect(
    db: &Arc<Db>,
    app_id: &String,
    event: &AppConnectEvent,
    creation_timestamp: &DateTime<Utc>,
) {
    // Establish a new transaction
    let mut tx = match db.connection_pool.begin().await {
        Ok(tx) => tx,
        Err(err) => {
            error!(
                "Failed to create new transaction to save app connection event, app_id: [{}], event: [{:?}], err: [{}]",
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
            &EventType::AppConnect,
            &creation_timestamp,
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
                    "Failed to rollback transaction for new app connection event, app_id: [{}], event: [{:?}], err: [{}]",
                    app_id, event, err
                );
            }

            return;
        }
    };

    // Now create a new event app connect corresponding to the event
    match db
        .create_new_event_app_connect(
            &mut tx,
            event_id,
            app_id,
            &event.session_id,
            &event.device_metadata,
            &event.language,
            &event.timezone,
            event.new_session,
            &creation_timestamp,
        )
        .await
    {
        Ok(_) => {
            // Commit the transaction
            if let Err(err) = tx.commit().await {
                error!(
                    "Failed to commit transaction for new app connection event, app_id: [{}], event: [{:?}], err: [{}]",
                    app_id, event, err
                );
            }
        }
        Err(err) => {
            error!(
                "Failed to create new app connection event, app_id: [{}], event: [{:?}], err: [{}]",
                app_id, event, err
            );

            // Rollback the transaction
            if let Err(err) = tx.rollback().await {
                error!(
                    "Failed to rollback transaction for new app connection event, app_id: [{}], event: [{:?}], err: [{}]",
                    app_id, event, err
                );
            }
        }
    }
}
