#[cfg(feature = "cloud_integration_tests")]
#[cfg(test)]
mod test {

    use crate::{
        db::Db,
        structs::{
            consts::{DAY_IN_SECONDS, HOUR_IN_SECONDS},
            device_metadata::{Device, DeviceMetadata, MobileMetadata, WebMetadata},
            event_type::EventType,
            request_status::RequestStatus,
            session_type::SessionType,
        },
        tables::utils::get_current_datetime,
    };
    use futures::future::join_all;
    use rand::Rng;
    use sqlx::types::chrono::{DateTime, Utc};
    use std::{sync::Arc, time::Duration};
    use tokio::task;

    #[tokio::test]
    async fn test_requests_success_rate() {
        let db = Db::connect_to_the_pool().await;
        // db.truncate_all_tables().await.unwrap();

        // Create test team instance
        // let team_id = "test_team_id".to_string();
        let app_id = "TEMPLATE_UID".to_string();

        let db_arc = Arc::new(db);
        let mut tasks: Vec<task::JoinHandle<()>> = Vec::new();
        let network = "Solana".to_string();

        let db_arc_clone = db_arc.clone();
        for i in 0..33 {
            let db_clone_clone = db_arc_clone.clone();
            let app_id = app_id.clone();
            let network = network.clone();
            tasks.push(task::spawn(async move {
                let mut tx = db_clone_clone
                    .connection_pool
                    .begin()
                    .await
                    .expect("Failed to start transaction");

                let current_timestamp = get_current_datetime();
                let total_events = 300 - i; // Total events per day
                let events_first_quarter = total_events / 7; // Approx 1/7th of events for first quarter
                let events_second_quarter = events_first_quarter * 2; // 2 times the first quarter

                for j in 0..total_events {
                    let base_time_offset = i * DAY_IN_SECONDS;
                    let additional_millis = (j + 1) * 100;

                    let (quarter_offset, n_th_value) = if j < events_first_quarter {
                        (0, 3)
                    } else if j < events_first_quarter + events_second_quarter {
                        (HOUR_IN_SECONDS / 4, 10)
                    } else {
                        (2 * HOUR_IN_SECONDS / 4, 2)
                    };

                    let creation_time: DateTime<Utc> = current_timestamp
                        - Duration::from_secs(base_time_offset + quarter_offset)
                        - Duration::from_millis(additional_millis);

                    // let status = if j % n_th_value == 0 {
                    //     RequestStatus::Completed
                    // } else {
                    //     RequestStatus::Pending
                    // };

                    let status = if rand::thread_rng().gen_range(0..=10) % 3 == 0 {
                        RequestStatus::Completed
                    } else {
                        RequestStatus::Pending
                    };

                    // Create new event
                    let event_id = db_clone_clone
                        .create_new_event_entry(
                            &mut tx,
                            &app_id,
                            &network,
                            &EventType::SignMessage,
                            &creation_time,
                        )
                        .await
                        .unwrap();

                    let request_id = format!("test_request_id_{}_{}", i, j);
                    db_clone_clone
                        .create_new_event_sign_message(
                            &mut tx,
                            event_id,
                            &"test_session_id".to_string(),
                            &request_id,
                            &"test_network".to_string(),
                        )
                        .await
                        .unwrap();

                    db_clone_clone
                        .update_event_sign_message(&mut tx, &request_id, status.clone())
                        .await
                        .unwrap();

                    // Create new event
                    let event_id = db_clone_clone
                        .create_new_event_entry(
                            &mut tx,
                            &app_id,
                            &network,
                            &EventType::SignTransaction,
                            &creation_time,
                        )
                        .await
                        .unwrap();

                    let request_id = format!("test_request_id_{}_{}_2", i, j);
                    db_clone_clone
                        .create_new_event_sign_transaction(
                            &mut tx,
                            event_id,
                            &"test_session_id".to_string(),
                            &request_id,
                            &"test_network".to_string(),
                        )
                        .await
                        .unwrap();

                    db_clone_clone
                        .update_event_sign_transaction(&mut tx, &request_id, status.clone(), &None)
                        .await
                        .unwrap();

                    // Create new event
                    let event_id = db_clone_clone
                        .create_new_event_entry(
                            &mut tx,
                            &app_id,
                            &network,
                            &EventType::SignAndSendTransaction,
                            &creation_time,
                        )
                        .await
                        .unwrap();

                    let request_id = format!("test_request_id_{}_{}_3", i, j);
                    db_clone_clone
                        .create_new_event_sign_and_send_transaction(
                            &mut tx,
                            event_id,
                            &"test_session_id".to_string(),
                            &request_id,
                            &"test_network".to_string(),
                        )
                        .await
                        .unwrap();

                    db_clone_clone
                        .update_event_sign_and_send_transaction(&mut tx, &request_id, status, &None)
                        .await
                        .unwrap();
                }

                tx.commit().await.expect("Failed to commit transaction");
            }));
        }

        // Await all tasks to complete
        join_all(tasks).await;

        // We need to refresh manually the views
        db_arc
            .refresh_continuous_aggregates(vec![
                "quarter_events_sign_message_stats_per_app".to_string(),
                "hour_events_sign_message_stats_per_app".to_string(),
                "daily_events_sign_message_stats_per_app".to_string(),
            ])
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_requests_change_success_rate() {
        let db = Db::connect_to_the_pool().await;
        // db.truncate_all_tables().await.unwrap();

        // Create test team instance
        // let team_id = "test_team_id".to_string();
        let app_id = "TEMPLATE_UID".to_string();

        let db_arc = Arc::new(db);
        let mut tasks: Vec<task::JoinHandle<()>> = Vec::new();
        let network = "Solana".to_string();

        let db_arc_clone = db_arc.clone();
        for i in 0..33 {
            let db_clone_clone = db_arc_clone.clone();
            let app_id = app_id.clone();
            let network = network.clone();

            tasks.push(task::spawn(async move {
                let mut tx = db_clone_clone
                    .connection_pool
                    .begin()
                    .await
                    .expect("Failed to start transaction");

                let current_timestamp = get_current_datetime();
                let total_events = 300 - i; // Total events per day
                let events_first_quarter = total_events / 7; // Approx 1/7th of events for first quarter
                let events_second_quarter = events_first_quarter * 2; // 2 times the first quarter

                let wallet_name = match i % 3 {
                    0 => "test_wallet_name_1",
                    1 => "test_wallet_name_2",
                    _ => "test_wallet_name_3",
                };

                for j in 0..total_events {
                    let base_time_offset = i * DAY_IN_SECONDS;
                    let additional_millis = (j + 1) * 100;

                    let (quarter_offset, n_th_value) = if j < events_first_quarter {
                        (0, 3)
                    } else if j < events_first_quarter + events_second_quarter {
                        (HOUR_IN_SECONDS / 4, 10)
                    } else {
                        (2 * HOUR_IN_SECONDS / 4, 2)
                    };

                    let creation_time: DateTime<Utc> = current_timestamp
                        - Duration::from_secs(base_time_offset + quarter_offset)
                        - Duration::from_millis(additional_millis);

                    // let status = if j % n_th_value == 0 {
                    //     RequestStatus::Completed
                    // } else {
                    //     RequestStatus::Pending
                    // };

                    let status = if rand::thread_rng().gen_range(0..=10) % 3 == 0 {
                        RequestStatus::Completed
                    } else {
                        RequestStatus::Pending
                    };

                    // Create new event
                    let event_id = db_clone_clone
                        .create_new_event_entry(
                            &mut tx,
                            &app_id,
                            &network,
                            &EventType::SignMessage,
                            &creation_time,
                        )
                        .await
                        .unwrap();

                    let request_id = format!("test_request_id_{}_{}_4", i, j);
                    db_clone_clone
                        .create_new_event_change_wallet(
                            &mut tx,
                            event_id,
                            &"test_session_id".to_string(),
                            &request_id,
                            &"test_network".to_string(),
                            &wallet_name.to_string(),
                            &"test_wallet_type".to_string(),
                            &"test_old_wallet_address".to_string(),
                        )
                        .await
                        .unwrap();

                    db_clone_clone
                        .update_event_change_wallet(&mut tx, &request_id, status.clone(), &None)
                        .await
                        .unwrap();
                }

                tx.commit().await.expect("Failed to commit transaction");
            }));
        }

        // Await all tasks to complete
        join_all(tasks).await;

        // We need to refresh manually the views
        db_arc
            .refresh_continuous_aggregates(vec![
                "quarter_events_change_wallet_stats_per_app".to_string(),
                "hour_events_change_wallet_stats_per_app".to_string(),
                "daily_events_change_wallet_stats_per_app".to_string(),
            ])
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_requests_change_network_success_rate() {
        let db = Db::connect_to_the_pool().await;
        // db.truncate_all_tables().await.unwrap();

        // Create test team instance
        // let team_id = "test_team_id".to_string();
        let app_id = "TEMPLATE_UID".to_string();

        let db_arc = Arc::new(db);
        let mut tasks: Vec<task::JoinHandle<()>> = Vec::new();
        let network = "Solana".to_string();

        let db_arc_clone = db_arc.clone();
        for i in 0..33 {
            let db_clone_clone = db_arc_clone.clone();
            let app_id = app_id.clone();
            let network = network.clone();

            tasks.push(task::spawn(async move {
                let mut tx = db_clone_clone
                    .connection_pool
                    .begin()
                    .await
                    .expect("Failed to start transaction");

                let current_timestamp = get_current_datetime();
                let total_events = 300 - i; // Total events per day
                let events_first_quarter = total_events / 7; // Approx 1/7th of events for first quarter
                let events_second_quarter = events_first_quarter * 2; // 2 times the first quarter

                let old_network = match i % 3 {
                    0 => "SOLANA",
                    1 => "APTOS",
                    _ => "NEAR",
                };

                for j in 0..total_events {
                    let base_time_offset = i * DAY_IN_SECONDS;
                    let additional_millis = (j + 1) * 100;

                    let (quarter_offset, n_th_value) = if j < events_first_quarter {
                        (0, 3)
                    } else if j < events_first_quarter + events_second_quarter {
                        (HOUR_IN_SECONDS / 4, 10)
                    } else {
                        (2 * HOUR_IN_SECONDS / 4, 2)
                    };

                    let creation_time: DateTime<Utc> = current_timestamp
                        - Duration::from_secs(base_time_offset + quarter_offset)
                        - Duration::from_millis(additional_millis);

                    // let status = if j % n_th_value == 0 {
                    //     RequestStatus::Completed
                    // } else {
                    //     RequestStatus::Pending
                    // };

                    let status = if rand::thread_rng().gen_range(0..=10) % 3 == 0 {
                        RequestStatus::Completed
                    } else {
                        RequestStatus::Pending
                    };

                    // Create new event
                    let event_id = db_clone_clone
                        .create_new_event_entry(
                            &mut tx,
                            &app_id,
                            &network,
                            &EventType::ChangeNetwork,
                            &creation_time,
                        )
                        .await
                        .unwrap();

                    let request_id = format!("test_request_id_{}_{}_4", i, j);
                    db_clone_clone
                        .create_new_event_change_network(
                            &mut tx,
                            event_id,
                            &"test_session_id".to_string(),
                            &request_id,
                            &old_network.to_string(),
                        )
                        .await
                        .unwrap();

                    db_clone_clone
                        .update_event_change_network(
                            &mut tx,
                            &request_id,
                            status.clone(),
                            &Some("MY NEW NETWORK".to_string()),
                        )
                        .await
                        .unwrap();
                }

                tx.commit().await.expect("Failed to commit transaction");
            }));
        }

        // Await all tasks to complete
        join_all(tasks).await;

        // We need to refresh manually the views
        db_arc
            .refresh_continuous_aggregates(vec![
                "quarter_events_change_network_stats_per_app".to_string(),
                "hour_events_change_network_stats_per_app".to_string(),
                "daily_events_change_network_stats_per_app".to_string(),
            ])
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_requests_client_connect_success_rate() {
        let db = Db::connect_to_the_pool().await;
        // db.truncate_all_tables().await.unwrap();

        // Create test team instance
        // let team_id = "test_team_id".to_string();
        let app_id = "TEMPLATE_UID".to_string();

        let db_arc = Arc::new(db);
        let mut tasks: Vec<task::JoinHandle<()>> = Vec::new();
        let network = "Solana".to_string();

        let db_arc_clone = db_arc.clone();
        for i in 0..33 {
            let db_clone_clone = db_arc_clone.clone();
            let app_id = app_id.clone();
            let network = network.clone();

            tasks.push(task::spawn(async move {
                let mut tx = db_clone_clone
                    .connection_pool
                    .begin()
                    .await
                    .expect("Failed to start transaction");

                let current_timestamp = get_current_datetime();
                let total_events = 100 - i; // Total events per day
                let events_first_quarter = total_events / 7; // Approx 1/7th of events for first quarter
                let events_second_quarter = events_first_quarter * 2; // 2 times the first quarter

                let wallet_name = match i % 3 {
                    0 => "test_wallet_name_1",
                    1 => "test_wallet_name_2",
                    _ => "test_wallet_name_3",
                };

                for j in 0..total_events {
                    let base_time_offset = i * DAY_IN_SECONDS;
                    let additional_millis = (j + 1) * 100;

                    let (quarter_offset, _n_th_value) = if j < events_first_quarter {
                        (0, 3)
                    } else if j < events_first_quarter + events_second_quarter {
                        (HOUR_IN_SECONDS / 4, 10)
                    } else {
                        (2 * HOUR_IN_SECONDS / 4, 2)
                    };

                    let creation_time: DateTime<Utc> = current_timestamp
                        - Duration::from_secs(base_time_offset + quarter_offset)
                        - Duration::from_millis(additional_millis);

                    let session_type = match j % 3 {
                        0 => SessionType::Extension,
                        _ => SessionType::Relay,
                    };

                    // Create new event
                    let event_id = db_clone_clone
                        .create_new_event_entry(
                            &mut tx,
                            &app_id,
                            &network,
                            &EventType::ChangeNetwork,
                            &creation_time,
                        )
                        .await
                        .unwrap();

                    let session_id = format!(
                        "TEST_SESSION_ID_{}_{}_{}",
                        base_time_offset, additional_millis, j
                    );
                    db_clone_clone
                        .create_new_event_client_connect(
                            &mut tx,
                            event_id,
                            &"TEST_CLIENT_ID".to_string(),
                            &session_id,
                            &wallet_name.to_string(),
                            &"test_wallet_type".to_string(),
                            &session_type,
                        )
                        .await
                        .unwrap();

                    // let success = j % 4 == 0;
                    let success = rand::thread_rng().gen_range(0..=10) % 3 == 0;

                    db_clone_clone
                        .update_event_client_connect(
                            &mut tx,
                            &"TEST_CLIENT_ID".to_string(),
                            &session_id,
                            success,
                            &vec![format!(
                                "NEW_ADDRESS_{}_{}_{}",
                                base_time_offset, additional_millis, j
                            )
                            .to_string()],
                        )
                        .await
                        .unwrap();
                }

                tx.commit().await.expect("Failed to commit transaction");
            }));
        }

        // Await all tasks to complete
        join_all(tasks).await;

        // We need to refresh manually the views
        db_arc
            .refresh_continuous_aggregates(vec![
                "quarter_events_client_connect_wallet_stats_per_app".to_string(),
                "hour_events_client_connect_wallet_stats_per_app".to_string(),
                "daily_events_client_connect_wallet_stats_per_app".to_string(),
            ])
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_requests_app_connect_success_rate() {
        let db = Db::connect_to_the_pool().await;
        // db.truncate_all_tables().await.unwrap();

        // Create test team instance
        // let team_id = "test_team_id".to_string();
        let app_id = "TEMPLATE_UID".to_string();

        let db_arc = Arc::new(db);
        let mut tasks: Vec<task::JoinHandle<()>> = Vec::new();
        let network = "Solana".to_string();

        let db_arc_clone = db_arc.clone();
        for i in 0..33 {
            let db_clone_clone = db_arc_clone.clone();
            let app_id = app_id.clone();
            let network = network.clone();

            tasks.push(task::spawn(async move {
                let mut tx = db_clone_clone
                    .connection_pool
                    .begin()
                    .await
                    .expect("Failed to start transaction");

                let current_timestamp = get_current_datetime();
                let total_events = 100 - i; // Total events per day
                let events_first_quarter = total_events / 7; // Approx 1/7th of events for first quarter
                let events_second_quarter = events_first_quarter * 2; // 2 times the first quarter

                for j in 0..total_events {
                    let base_time_offset = i * DAY_IN_SECONDS;
                    let additional_millis = (j + 1) * 100;

                    let (quarter_offset, _n_th_value) = if j < events_first_quarter {
                        (0, 3)
                    } else if j < events_first_quarter + events_second_quarter {
                        (HOUR_IN_SECONDS / 4, 10)
                    } else {
                        (2 * HOUR_IN_SECONDS / 4, 2)
                    };

                    let creation_time: DateTime<Utc> = current_timestamp
                        - Duration::from_secs(base_time_offset + quarter_offset)
                        - Duration::from_millis(additional_millis);

                    let language = match j % 6 {
                        0 | 1 | 2 => "English".to_string(),
                        3 | 4 => "French".to_string(),
                        _ => "German".to_string(),
                    };

                    let device_metadata = match j % 7 {
                        0 => {
                            let browser = if j % 2 == 0 {
                                "Chrome".to_string()
                            } else {
                                "Firefox".to_string()
                            };

                            let os = if j % 3 == 0 {
                                "Windows".to_string()
                            } else {
                                if rand::thread_rng().gen_range(0..=10) % 2 == 0 {
                                    "Linux".to_string()
                                } else {
                                    "Mac".to_string()
                                }
                            };

                            DeviceMetadata::Web(WebMetadata {
                                browser: browser,
                                os: os,
                                os_version: "test_os_version".to_string(),
                                browser_version: "test_version".to_string(),
                            })
                        }
                        1 | 2 | 3 | 4 | 5 => {
                            let rng = rand::thread_rng().gen::<i8>();

                            let system = match rng % 3 {
                                0 => Device::Android,
                                1 => Device::Apple,
                                _ => Device::Unknown,
                            };

                            DeviceMetadata::Mobile(MobileMetadata {
                                system: system,
                                version: "test_version".to_string(),
                            })
                        }
                        _ => DeviceMetadata::Unknown,
                    };

                    // Create new event
                    let event_id = db_clone_clone
                        .create_new_event_entry(
                            &mut tx,
                            &app_id,
                            &network,
                            &EventType::ChangeNetwork,
                            &creation_time,
                        )
                        .await
                        .unwrap();

                    let session_id = format!(
                        "TEST_SESSION_ID_{}_{}_{}",
                        base_time_offset, additional_millis, j
                    );
                    db_clone_clone
                        .create_new_event_app_connect(
                            &mut tx,
                            event_id,
                            &app_id,
                            &network,
                            &session_id,
                            &device_metadata,
                            &language,
                            &"test_timezone".to_string(),
                            true,
                            &creation_time,
                        )
                        .await
                        .unwrap();
                }

                tx.commit().await.expect("Failed to commit transaction");
            }));
        }

        // Await all tasks to complete
        join_all(tasks).await;

        // We need to refresh manually the views
        db_arc
            .refresh_continuous_aggregates(vec![
                "quarter_events_app_connect_language_stats_per_app".to_string(),
                "hour_events_app_connect_language_stats_per_app".to_string(),
                "daily_events_app_connect_language_stats_per_app".to_string(),
                ////////////////////////////////////////////////////////////////
                "quarter_events_app_connect_browser_stats_per_app".to_string(),
                "hour_events_app_connect_browser_stats_per_app".to_string(),
                "daily_events_app_connect_browser_stats_per_app".to_string(),
                ////////////////////////////////////////////////////////////////
                "quarter_events_app_connect_os_stats_per_app".to_string(),
                "hour_events_app_connect_os_stats_per_app".to_string(),
                "daily_events_app_connect_os_stats_per_app".to_string(),
                ////////////////////////////////////////////////////////////////
                "quarter_events_app_connect_system_stats_per_app".to_string(),
                "hour_events_app_connect_system_stats_per_app".to_string(),
                "daily_events_app_connect_system_stats_per_app".to_string(),
                ////////////////////////////////////////////////////////////////
                "quarter_events_app_connect_system_stats_per_app".to_string(),
                "hour_events_app_connect_system_stats_per_app".to_string(),
                "daily_events_app_connect_system_stats_per_app".to_string(),
                ////////////////////////////////////////////////////////////////
                "quarter_events_app_connect_session_type_stats_per_app".to_string(),
                "hour_events_app_connect_session_type_stats_per_app".to_string(),
                "daily_events_app_connect_session_type_stats_per_app".to_string(),
            ])
            .await
            .unwrap();
    }
}
