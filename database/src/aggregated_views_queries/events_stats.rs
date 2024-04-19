// use crate::{
//     db::Db,
//     structs::{db_error::DbError, filter_requests::RequestsStats, time_filters::TimeFilter},
//     tables::utils::{format_view_keys, format_view_name},
// };

// pub const REQUESTS_STATS_BASE_VIEW_NAME: &str = "requests_stats_per_app";
// pub const REQUESTS_STATS_BASE_KEYS: [(&'static str, bool); 4] = [
//     ("app_id", false),
//     ("bucket", true),
//     ("request_count", true),
//     ("success_rate", true),
// ];

// impl Db {
//     pub async fn get_requests_stats_by_app_id(
//         &self,
//         app_id: &str,
//         filter: TimeFilter,
//     ) -> Result<Vec<RequestsStats>, DbError> {
//         let start_date = filter.to_date();
//         let bucket_size = filter.bucket_size();

//         // Correctly selecting the view based on the bucket_size
//         let prefix = match bucket_size {
//             "1 hour" => "hourly",
//             "1 day" => "daily",
//             "1 month" => "monthly",
//             _ => return Err(DbError::DatabaseError("Invalid bucket size".to_string())),
//         };

//         let formatted_keys = format_view_keys(prefix, &REQUESTS_STATS_BASE_KEYS);
//         let formatted_view_name = format_view_name(prefix, REQUESTS_STATS_BASE_VIEW_NAME);
//         let filter_key = REQUESTS_STATS_BASE_KEYS[1].0;
//         let filter = format!("{prefix}_{filter_key}");

//         let query = format!(
//             "SELECT {formatted_keys}
//             FROM {formatted_view_name}
//             WHERE app_id = $1 AND {filter} >= $2
//             ORDER BY {filter} DESC",
//         );

//         sqlx::query_as::<_, RequestsStats>(&query)
//             .bind(app_id)
//             .bind(start_date)
//             .fetch_all(&self.connection_pool)
//             .await
//             .map_err(|e| e.into())
//     }
// }

#[cfg(feature = "cloud_db_tests")]
#[cfg(test)]
mod test {

    use super::*;
    use crate::{
        db::Db,
        structs::{
            consts::{DAY_IN_SECONDS, HOUR_IN_SECONDS},
            event_type::EventType,
            request_status::RequestStatus,
            request_type::RequestType,
            session_type::SessionType,
        },
        tables::{
            registered_app::table_struct::DbRegisteredApp,
            sessions::table_struct::DbNcSession,
            utils::{get_current_datetime, to_microsecond_precision},
        },
    };
    use futures::future::join_all;
    use rand::{thread_rng, Rng};
    use sqlx::types::chrono::{DateTime, Utc};
    use std::{sync::Arc, time::Duration};
    use tokio::{sync::Semaphore, task};

    // #[tokio::test]
    // async fn test_generate_events_data() {
    //     let db = Db::connect_to_the_pool().await;
    //     db.truncate_all_tables().await.unwrap();

    //     // Create test team instance
    //     let team_id = "test_team_id".to_string();
    //     let app_id_2 = "test_app_id_2".to_string();

    //     let db_arc = Arc::new(db);
    //     let mut tasks = Vec::new();

    //     let now = get_current_datetime();
    //     let start_date = now - Duration::from_secs(60 * 60 * 24 * 60); // Start of second period, 60 days ago
    //     let end_date = now;
    //     let num_days = (end_date - start_date).num_days();

    //     let semaphore = Arc::new(Semaphore::new(8));
    //     let mut handles = vec![];

    //     for day_offset in 0..num_days {
    //         let app_id_clone = app_id_2.clone();
    //         let db_arc_clone = db_arc.clone();
    //         let sem_clone = semaphore.clone();

    //         let handle = tokio::task::spawn(async move {
    //             let _permit = sem_clone
    //                 .acquire()
    //                 .await
    //                 .expect("Failed to acquire semaphore permit");

    //             let day_start = start_date + Duration::from_secs(60 * 60 * 24 * day_offset as u64);

    //             // Loop through each hour of the day
    //             for hour in 0..24 {
    //                 let hour_start = day_start + Duration::from_secs(60 * 60 * hour as u64);
    //                 let events_count = if is_seventh_day {
    //                     // For the 7th day, limit sessions to 3-6 per hour
    //                     rand::thread_rng().gen_range(3..=6)
    //                 } else if hour >= 6 && hour <= 19 {
    //                     // Regular daytime hours
    //                     thread_rng().gen_range(20..=40)
    //                 } else {
    //                     // Regular nighttime hours
    //                     rand::thread_rng().gen_range(10..=20)
    //                 };

    //                 // Generate sessions for this hour
    //                 for i in 0..session_count {
    //                     let session_start =
    //                         hour_start + Duration::from_secs(rand::thread_rng().gen_range(0..3600));
    //                     let session_end = session_start + Duration::from_secs(600); // 10 minutes

    //                     let session = DbNcSession {
    //                         session_id: uuid7::uuid7().to_string(),
    //                         app_id: app_id_clone.clone(),
    //                         app_metadata: "test_metadata".to_string(),
    //                         persistent: false,
    //                         network: "Solana".to_string(),
    //                         client_data: None,
    //                         session_open_timestamp: session_start.clone(),
    //                         session_close_timestamp: Some(session_end),
    //                     };

    //                     let ip_address = "127.0.0.1".to_string();

    //                     db_arc_clone
    //                         .handle_new_session(&session, None, &ip_address, &session_start)
    //                         .await
    //                         .unwrap();

    //                     // let amount = thread_rng().gen_range(0..=10);
    //                     // // Generate random amount of additional connections from user
    //                     // for _ in 0..amount {
    //                     //     let mut tx = db_arc_clone.connection_pool.begin().await.unwrap();
    //                     //     db_arc_clone
    //                     //         .create_new_connection_event_by_client(
    //                     //             &mut tx,
    //                     //             &session.app_id,
    //                     //             &session.session_id,
    //                     //             &SessionType::Relay,
    //                     //             &ip_address,
    //                     //             None,
    //                     //             &(session_start + Duration::from_secs(1)),
    //                     //         )
    //                     //         .await
    //                     //         .unwrap();
    //                     //     tx.commit().await.unwrap();
    //                     // }

    //                     // db_arc_clone
    //                     //     .close_session(&session.session_id, session_end)
    //                     //     .await
    //                     //     .unwrap();
    //                 }
    //             }
    //         });

    //         handles.push(handle);
    //     }

    //     // Wait for all tasks to complete
    //     join_all(handles).await;

    //     // Manually refresh the continuous aggregates
    //     db_arc
    //         .refresh_continuous_aggregates(vec![
    //             "quarter_sessions_stats_per_app".to_string(),
    //             "hourly_sessions_stats_per_app".to_string(),
    //             "daily_sessions_stats_per_app".to_string(),
    //             // "monthly_sessions_stats_per_app".to_string(),
    //         ])
    //         .await
    //         .unwrap();

    //     // for i in 0..33 {
    //     //     let db_clone = db_arc.clone(); // Clone the db connection or pool if needed
    //     //     let app_id = app_id.clone();
    //     //     tasks.push(task::spawn(async move {
    //     //         for j in 0..100 - i {
    //     //             let creation_time: DateTime<Utc> = Utc::now()
    //     //                 - Duration::from_secs(i as u64 * DAY_IN_SECONDS as u64)
    //     //                 - Duration::from_millis((j + 1) as u64 * 100);

    //     //             let request = Request {
    //     //                 request_id: format!("test_request_id_{}_{}", i, j),
    //     //                 app_id: app_id.to_string(),
    //     //                 session_id: "test_session_id".to_string(),
    //     //                 network: "test_network".to_string(),
    //     //                 creation_timestamp: creation_time,
    //     //                 request_status: RequestStatus::Pending,
    //     //                 request_type: RequestType::SignAndSendTransaction,
    //     //             };

    //     //             if let Err(e) = db_clone.save_request(&request).await {
    //     //                 eprintln!("Failed to save request: {}", e);
    //     //             }
    //     //         }
    //     //     }));
    //     // }

    //     // // Await all tasks to complete
    //     // for task in tasks {
    //     //     task.await.unwrap();
    //     // }

    //     // // We need to refresh manually the views
    //     // db_arc
    //     //     .refresh_continuous_aggregates(vec![
    //     //         "hourly_requests_stats_per_app".to_string(),
    //     //         "daily_requests_stats_per_app".to_string(),
    //     //         "monthly_requests_stats_per_app".to_string(),
    //     //     ])
    //     //     .await
    //     //     .unwrap();

    //     // let result = db_arc
    //     //     .get_requests_stats_by_app_id(&app_id, TimeFilter::Last24Hours)
    //     //     .await
    //     //     .unwrap();

    //     // assert_eq!(result.len(), 2);
    //     // assert_eq!(result[0].request_count, 100);
    //     // assert_eq!(result[1].request_count, 99);

    //     // let result = db_arc
    //     //     .get_requests_stats_by_app_id(&app_id, TimeFilter::Last7Days)
    //     //     .await
    //     //     .unwrap();

    //     // assert_eq!(result.len(), 8);
    //     // assert_eq!(result[0].request_count, 100);
    //     // assert_eq!(result[7].request_count, 93);

    //     // let result = db_arc
    //     //     .get_requests_stats_by_app_id(&app_id, TimeFilter::Last30Days)
    //     //     .await
    //     //     .unwrap();

    //     // assert_eq!(result.len(), 31);
    //     // assert_eq!(result[0].request_count, 100);
    //     // assert_eq!(result[30].request_count, 70);
    // }

    #[tokio::test]
    async fn test_requests_success_rate() {
        let db = Db::connect_to_the_pool().await;
        // db.truncate_all_tables().await.unwrap();

        // Create test team instance
        // let team_id = "test_team_id".to_string();
        let app_id = "test_app_id_2".to_string();

        let db_arc = Arc::new(db);
        let mut tasks: Vec<task::JoinHandle<()>> = Vec::new();

        let db_arc_clone = db_arc.clone();
        for i in 0..33 {
            let db_clone_clone = db_arc_clone.clone();
            let app_id = app_id.clone();
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

                    let status = if j % n_th_value == 0 {
                        RequestStatus::Completed
                    } else {
                        RequestStatus::Pending
                    };

                    // Create new event
                    let event_id = db_clone_clone
                        .create_new_event_entry(
                            &mut tx,
                            &app_id,
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
        let app_id = "test_app_id_2".to_string();

        let db_arc = Arc::new(db);
        let mut tasks: Vec<task::JoinHandle<()>> = Vec::new();

        let db_arc_clone = db_arc.clone();
        for i in 0..33 {
            let db_clone_clone = db_arc_clone.clone();
            let app_id = app_id.clone();
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

                    let status = if j % n_th_value == 0 {
                        RequestStatus::Completed
                    } else {
                        RequestStatus::Pending
                    };

                    // Create new event
                    let event_id = db_clone_clone
                        .create_new_event_entry(
                            &mut tx,
                            &app_id,
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
        let app_id = "test_app_id_2".to_string();

        let db_arc = Arc::new(db);
        let mut tasks: Vec<task::JoinHandle<()>> = Vec::new();

        let db_arc_clone = db_arc.clone();
        for i in 0..33 {
            let db_clone_clone = db_arc_clone.clone();
            let app_id = app_id.clone();
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

                    let status = if j % n_th_value == 0 {
                        RequestStatus::Completed
                    } else {
                        RequestStatus::Pending
                    };

                    // Create new event
                    let event_id = db_clone_clone
                        .create_new_event_entry(
                            &mut tx,
                            &app_id,
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
        let app_id = "test_app_id_2".to_string();

        let db_arc = Arc::new(db);
        let mut tasks: Vec<task::JoinHandle<()>> = Vec::new();

        let db_arc_clone = db_arc.clone();
        for i in 0..33 {
            let db_clone_clone = db_arc_clone.clone();
            let app_id = app_id.clone();
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

                    let success = j % 4 == 0;

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
}
