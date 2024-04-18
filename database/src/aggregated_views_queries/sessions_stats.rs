use crate::{
    db::Db,
    structs::{db_error::DbError, filter_requests::SessionsStats, time_filters::TimeFilter},
    tables::utils::{format_view_keys, format_view_name},
};

pub const SESSIONS_STATS_BASE_VIEW_NAME: &str = "sessions_stats_per_app";
pub const SESSIONS_STATS_BASE_KEYS: [(&'static str, bool); 4] = [
    ("app_id", false),
    ("bucket", true),
    ("sessions_opened", true),
    ("active_users", true),
];

impl Db {
    pub async fn get_sessions_stats_by_app_id(
        &self,
        app_id: &str,
        filter: TimeFilter,
    ) -> Result<Vec<SessionsStats>, DbError> {
        let start_date = filter.to_date();
        let bucket_size = filter.bucket_size();

        // Correctly selecting the view based on the bucket_size
        let prefix = match bucket_size {
            "1 hour" => "hourly",
            "1 day" => "daily",
            "1 month" => "monthly",
            _ => return Err(DbError::DatabaseError("Invalid bucket size".to_string())),
        };

        let formatted_keys = format_view_keys(prefix, &SESSIONS_STATS_BASE_KEYS);
        let formatted_view_name = format_view_name(prefix, SESSIONS_STATS_BASE_VIEW_NAME);
        let filter_key = SESSIONS_STATS_BASE_KEYS[1].0;
        let filter = format!("{prefix}_{filter_key}");

        let query = format!(
            "SELECT {formatted_keys}
            FROM {formatted_view_name}
            WHERE app_id = $1 AND {filter} >= $2
            ORDER BY {filter} DESC",
        );

        sqlx::query_as::<_, SessionsStats>(&query)
            .bind(app_id)
            .bind(start_date)
            .fetch_all(&self.connection_pool)
            .await
            .map_err(|e| e.into())
    }
}

#[cfg(feature = "cloud_db_tests")]
#[cfg(test)]
mod tests {
    use crate::structs::session_type::SessionType;
    use crate::tables::sessions::table_struct::SESSIONS_TABLE_NAME;
    use crate::{
        structs::{
            client_data::ClientData, privilege_level::PrivilegeLevel, time_filters::TimeFilter,
        },
        tables::{
            registered_app::table_struct::DbRegisteredApp,
            sessions::table_struct::DbNcSession,
            team::table_struct::Team,
            user_app_privileges::table_struct::UserAppPrivilege,
            utils::{get_current_datetime, to_microsecond_precision},
        },
    };
    use futures::future::join_all;
    use rand::{thread_rng, Rng};
    use sqlx::query;
    use sqlx::{types::chrono::Utc, Transaction};
    use std::{sync::Arc, time::Duration};
    use tokio::sync::Semaphore;

    #[tokio::test]
    async fn test_sessions_count() {
        let db = super::Db::connect_to_the_pool().await;
        db.truncate_all_tables().await.unwrap();

        // Create test team instance
        let now = Utc::now();
        let start_of_period = now;
        let team_id = "test_team_id".to_string();
        let app_id = "test_app_id".to_string();

        db.setup_test_team(&team_id, &app_id, start_of_period)
            .await
            .unwrap();

        // Number of sessions to create
        let num_sessions: u64 = 100;

        // Generate and save sessions
        for i in 0..num_sessions {
            // spread sessions evenly across the day
            let session_start =
                start_of_period + Duration::from_secs(i * 3600 / num_sessions as u64);
            let session_end = session_start + Duration::from_secs(60 * 10); // duration of 10 minutes for each session

            let session = DbNcSession {
                session_id: format!("session_{}_{}", app_id, i),
                app_id: app_id.clone(),
                app_metadata: "test_metadata".to_string(),
                persistent: false,
                network: "test_network".to_string(),
                client_data: None,
                session_open_timestamp: session_start.clone(),
                session_close_timestamp: None,
            };

            db.handle_new_session(&session, None, &"127.0.0.1".to_string(), &session_start)
                .await
                .unwrap();
            db.close_session(&session.session_id, session_end)
                .await
                .unwrap();
        }

        // Manually refresh the continuous aggregates
        db.refresh_continuous_aggregates(vec![
            "hourly_sessions_stats_per_app".to_string(),
            "daily_sessions_stats_per_app".to_string(),
            // "monthly_sessions_stats_per_app".to_string(),
        ])
        .await
        .unwrap();

        let stats = db
            .get_sessions_stats_by_app_id(&app_id, TimeFilter::LastMonth)
            .await
            .unwrap();

        assert_eq!(stats.len(), 1);
        assert_eq!(stats[0].sessions_opened, num_sessions as i64);
    }

    #[tokio::test]
    async fn test_sessions_average_daily_sessions_opened() {
        let db = super::Db::connect_to_the_pool().await;
        db.truncate_all_tables().await.unwrap();

        // Create test team instance
        let team_id = "test_team_id".to_string();
        let app_id = "test_app_id".to_string();

        db.setup_test_team(&team_id, &app_id, Utc::now())
            .await
            .unwrap();

        let now = Utc::now();
        let start_of_first_period = now - Duration::from_secs(60 * 60 * 24 * 60); // Start of first period, 60 days ago
        let start_of_second_period = now - Duration::from_secs(60 * 60 * 24 * 30); // Start of second period, 30 days ago
        let num_sessions_first_period: u64 = 40;
        let num_sessions_second_period: u64 = 28;

        // Generate and save sessions for the first period
        for i in 0..num_sessions_first_period {
            let session_start = start_of_first_period
                + Duration::from_secs(i * 86400 / num_sessions_first_period as u64);
            let session_end = session_start + Duration::from_secs(60 * 30); // Duration of 30 minutes for each session

            let session = DbNcSession {
                session_id: format!("session_{}_{}", app_id, i),
                app_id: app_id.clone(),
                app_metadata: "test_metadata".to_string(),
                persistent: false,
                network: "test_network".to_string(),
                client_data: None,
                session_open_timestamp: session_start.clone(),
                session_close_timestamp: Some(session_end),
            };

            db.handle_new_session(&session, None, &"127.0.0.1".to_string(), &session_start)
                .await
                .unwrap();
            db.close_session(&session.session_id, session_end)
                .await
                .unwrap();
        }

        // Generate and save sessions for the second period
        for i in 0..num_sessions_second_period {
            let session_start = start_of_second_period
                + Duration::from_secs(i * 86400 / num_sessions_second_period as u64);
            let session_end = session_start + Duration::from_secs(60 * 30); // Duration of 30 minutes for each session

            let session = DbNcSession {
                session_id: format!("session_{}_{}_2nd", app_id, i), // Ensure unique session IDs for the second period
                app_id: app_id.clone(),
                app_metadata: "test_metadata".to_string(),
                persistent: false,
                network: "test_network".to_string(),
                client_data: None,
                session_open_timestamp: session_start.clone(),
                session_close_timestamp: Some(session_end),
            };

            db.handle_new_session(&session, None, &"127.0.0.1".to_string(), &session_start)
                .await
                .unwrap();
            db.close_session(&session.session_id, session_end)
                .await
                .unwrap();
        }

        // Manually refresh the continuous aggregates
        db.refresh_continuous_aggregates(vec![
            "hourly_sessions_stats_per_app".to_string(),
            "daily_sessions_stats_per_app".to_string(),
            // "monthly_sessions_stats_per_app".to_string(),
        ])
        .await
        .unwrap();

        let stats = db
            .get_sessions_stats_by_app_id(&app_id, TimeFilter::Last30Days)
            .await
            .unwrap();

        assert_eq!(stats.len(), 2);
    }

    #[tokio::test]
    async fn test_sessions_active_users() {
        let db = super::Db::connect_to_the_pool().await;
        db.truncate_all_tables().await.unwrap();

        // Create test team instance
        let team_id = "test_team_id".to_string();
        let app_id = "test_app_id".to_string();

        db.setup_test_team(&team_id, &app_id, Utc::now())
            .await
            .unwrap();

        let now = Utc::now();
        let start_of_first_period = now - Duration::from_secs(60 * 60 * 24 * 60); // Start of first period, 60 days ago
        let start_of_second_period = now - Duration::from_secs(60 * 60 * 24 * 30); // Start of second period, 30 days ago
        let num_sessions_first_period: u64 = 40;
        let num_sessions_second_period: u64 = 28;

        // Generate and save sessions for the first period
        for i in 0..num_sessions_first_period {
            let session_start = start_of_first_period
                + Duration::from_secs(i * 86400 / num_sessions_first_period as u64);
            let session_end = session_start + Duration::from_secs(60 * 30); // Duration of 30 minutes for each session

            let session_id = format!("session_{}_{}", app_id, i);
            let session = DbNcSession {
                session_id: session_id.clone(),
                app_id: app_id.clone(),
                app_metadata: "test_metadata".to_string(),
                persistent: false,
                network: "test_network".to_string(),
                client_data: None,
                session_open_timestamp: session_start.clone(),
                session_close_timestamp: Some(session_end),
            };

            db.handle_new_session(&session, None, &"127.0.0.1".to_string(), &session_start)
                .await
                .unwrap();

            {
                let data = ClientData {
                    client_id: "test_client_id".to_string(),
                    wallet_name: "test_wallet_name".to_string(),
                    wallet_type: "test_wallet_type".to_string(),
                    client_profile_id: 1,
                    connected_at: session_start,
                };

                // Update the session with the client data
                let query_body = format!(
                    "UPDATE {SESSIONS_TABLE_NAME} SET client_data = $1 WHERE session_id = $2"
                );

                query(&query_body)
                    .bind(&data)
                    .bind(&session.session_id)
                    .execute(&db.connection_pool)
                    .await
                    .unwrap();
            }

            db.close_session(&session.session_id, session_end)
                .await
                .unwrap();
        }

        // Generate and save sessions for the second period
        for i in 0..num_sessions_second_period {
            let session_start = start_of_second_period
                + Duration::from_secs(i * 86400 / num_sessions_second_period as u64);
            let session_end = session_start + Duration::from_secs(60 * 30); // Duration of 30 minutes for each session

            let session = DbNcSession {
                session_id: format!("session_{}_{}_2nd", app_id, i), // Ensure unique session IDs for the second period
                app_id: app_id.clone(),
                app_metadata: "test_metadata".to_string(),
                persistent: false,
                network: "test_network".to_string(),
                client_data: None,
                session_open_timestamp: session_start.clone(),
                session_close_timestamp: Some(session_end),
            };

            db.handle_new_session(&session, None, &"127.0.0.1".to_string(), &session_start)
                .await
                .unwrap();

            {
                let data = ClientData {
                    client_id: "test_client_id".to_string(),
                    wallet_name: "test_wallet_name".to_string(),
                    wallet_type: "test_wallet_type".to_string(),
                    client_profile_id: 1,
                    connected_at: session_start,
                };

                // Update the session with the client data
                let query_body = format!(
                    "UPDATE {SESSIONS_TABLE_NAME} SET client_data = $1 WHERE session_id = $2"
                );

                query(&query_body)
                    .bind(&data)
                    .bind(&session.session_id)
                    .execute(&db.connection_pool)
                    .await
                    .unwrap();
            }

            db.close_session(&session.session_id, session_end)
                .await
                .unwrap();
        }

        // Manually refresh the continuous aggregates
        db.refresh_continuous_aggregates(vec![
            "quarter_sessions_stats_per_app".to_string(),
            "hourly_sessions_stats_per_app".to_string(),
            "daily_sessions_stats_per_app".to_string(),
            // "monthly_sessions_stats_per_app".to_string(),
        ])
        .await
        .unwrap();

        let stats = db
            .get_sessions_stats_by_app_id(&app_id, TimeFilter::Last7Days)
            .await
            .unwrap();

        println!("stats {:?}", stats);
        // assert_eq!(stats.len(), 2);
    }

    #[tokio::test]
    async fn generate_session_stats_1() {
        let db = super::Db::connect_to_the_pool().await;
        db.truncate_all_tables().await.unwrap();

        let team_id = "test_team_id".to_string();
        let app_id = "test_app_id".to_string();

        db.setup_test_team(&team_id, &app_id, get_current_datetime())
            .await
            .unwrap();

        // let now = get_current_datetime();
        // let start_date = now - Duration::from_secs(60 * 60 * 24 * 60); // Start of second period, 60 days ago
        // let end_date = now;
        // let num_days = (end_date - start_date).num_days();

        // let semaphore = Arc::new(Semaphore::new(8));
        // let mut handles = vec![];
        // let db_arc = Arc::new(db);

        // for day_offset in 0..num_days {
        //     let app_id_clone = app_id.clone();
        //     let db_arc_clone = db_arc.clone();
        //     let sem_clone = semaphore.clone();

        //     let handle = tokio::task::spawn(async move {
        //         let _permit = sem_clone
        //             .acquire()
        //             .await
        //             .expect("Failed to acquire semaphore permit");

        //         let day_start = start_date + Duration::from_secs(60 * 60 * 24 * day_offset as u64);

        //         let is_seventh_day = day_offset % 7 == 6; // day_offset is 0-based; 6 represents the 7th day

        //         // Loop through each hour of the day
        //         for hour in 0..24 {
        //             let hour_start = day_start + Duration::from_secs(60 * 60 * hour as u64);
        //             let session_count = if is_seventh_day {
        //                 // For the 7th day, limit sessions to 3-6 per hour
        //                 rand::thread_rng().gen_range(3..=6)
        //             } else if hour >= 6 && hour <= 19 {
        //                 // Regular daytime hours
        //                 thread_rng().gen_range(20..=40)
        //             } else {
        //                 // Regular nighttime hours
        //                 rand::thread_rng().gen_range(10..=20)
        //             };

        //             // Generate sessions for this hour
        //             for _ in 0..session_count {
        //                 let session_start =
        //                     hour_start + Duration::from_secs(rand::thread_rng().gen_range(0..3600));
        //                 let session_end = session_start + Duration::from_secs(600); // 10 minutes

        //                 let session = DbNcSession {
        //                     session_id: uuid7::uuid7().to_string(),
        //                     app_id: app_id_clone.clone(),
        //                     app_metadata: "test_metadata".to_string(),
        //                     persistent: false,
        //                     network: "Solana".to_string(),
        //                     client_data: None,
        //                     session_open_timestamp: session_start,
        //                     session_close_timestamp: Some(session_end),
        //                 };

        //                 db_arc_clone
        //                     .handle_new_session(&session, None, &"127.0.0.1".to_string())
        //                     .await
        //                     .unwrap();
        //                 db_arc_clone
        //                     .close_session(&session.session_id, session_end)
        //                     .await
        //                     .unwrap();
        //             }
        //         }
        //     });

        //     handles.push(handle);
        // }

        // // Wait for all tasks to complete
        // join_all(handles).await;

        // // Manually refresh the continuous aggregates
        // db_arc
        //     .refresh_continuous_aggregates(vec![
        //         "quarter_sessions_stats_per_app".to_string(),
        //         "hourly_sessions_stats_per_app".to_string(),
        //         "daily_sessions_stats_per_app".to_string(),
        //         // "monthly_sessions_stats_per_app".to_string(),
        //     ])
        //     .await
        //     .unwrap();
    }

    #[tokio::test]
    async fn generate_session_stats_2() {
        let db = super::Db::connect_to_the_pool().await;

        let team_id = "test_team_id".to_string();

        let now = get_current_datetime();
        let start_date = now - Duration::from_secs(60 * 60 * 24 * 60); // Start of second period, 60 days ago
        let end_date = now;
        let num_days = (end_date - start_date).num_days();

        let db_arc = Arc::new(db);

        // Add second app
        let app_id_2 = "test_app_id_2".to_string();
        let registered_app = DbRegisteredApp {
            team_id: team_id.clone(),
            app_id: app_id_2.clone(),
            app_name: format!("{app_id_2}_APP_NAME").to_string(),
            whitelisted_domains: vec!["localhost".to_string()],
            ack_public_keys: vec!["key".to_string()],
            registration_timestamp: get_current_datetime(),
        };

        let mut tx = db_arc.connection_pool.begin().await.unwrap();
        db_arc
            .register_new_app_within_tx(&mut tx, &registered_app)
            .await
            .unwrap();
        tx.commit().await.unwrap();

        // let semaphore = Arc::new(Semaphore::new(8));
        // let mut handles = vec![];

        // for day_offset in 0..num_days {
        //     let app_id_clone = app_id_2.clone();
        //     let db_arc_clone = db_arc.clone();
        //     let sem_clone = semaphore.clone();

        //     let handle = tokio::task::spawn(async move {
        //         let _permit = sem_clone
        //             .acquire()
        //             .await
        //             .expect("Failed to acquire semaphore permit");

        //         let day_start = start_date + Duration::from_secs(60 * 60 * 24 * day_offset as u64);

        //         let is_seventh_day = day_offset % 7 == 6; // day_offset is 0-based; 6 represents the 7th day

        //         // Loop through each hour of the day
        //         for hour in 0..24 {
        //             let hour_start = day_start + Duration::from_secs(60 * 60 * hour as u64);
        //             let session_count = if is_seventh_day {
        //                 // For the 7th day, limit sessions to 3-6 per hour
        //                 rand::thread_rng().gen_range(3..=6)
        //             } else if hour >= 6 && hour <= 19 {
        //                 // Regular daytime hours
        //                 thread_rng().gen_range(20..=40)
        //             } else {
        //                 // Regular nighttime hours
        //                 rand::thread_rng().gen_range(10..=20)
        //             };

        //             // Generate sessions for this hour
        //             for i in 0..session_count {
        //                 let session_start =
        //                     hour_start + Duration::from_secs(rand::thread_rng().gen_range(0..3600));
        //                 let session_end = session_start + Duration::from_secs(600); // 10 minutes

        //                 let session = DbNcSession {
        //                     session_id: uuid7::uuid7().to_string(),
        //                     app_id: app_id_clone.clone(),
        //                     app_metadata: "test_metadata".to_string(),
        //                     persistent: false,
        //                     network: "Solana".to_string(),
        //                     client_data: None,
        //                     session_open_timestamp: session_start.clone(),
        //                     session_close_timestamp: Some(session_end),
        //                 };

        //                 let ip_address = "127.0.0.1".to_string();

        //                 db_arc_clone
        //                     .handle_new_session(&session, None, &ip_address, &session_start)
        //                     .await
        //                     .unwrap();

        //                 if i % 3 != 0 {
        //                     let data = ClientData {
        //                         client_id: "test_client_id".to_string(),
        //                         wallet_name: "test_wallet_name".to_string(),
        //                         wallet_type: "test_wallet_type".to_string(),
        //                         client_profile_id: session_start.timestamp(),
        //                         connected_at: session_start,
        //                     };

        //                     // Update the session with the client data
        //                     let query_body = format!(
        //                             "UPDATE {SESSIONS_TABLE_NAME} SET client_data = $1 WHERE session_id = $2"
        //                         );

        //                     query(&query_body)
        //                         .bind(&data)
        //                         .bind(&session.session_id)
        //                         .execute(&db_arc_clone.connection_pool)
        //                         .await
        //                         .unwrap();
        //                 }

        //                 let amount = thread_rng().gen_range(0..=10);
        //                 // Generate random amount of additional connections from user
        //                 for _ in 0..amount {
        //                     let mut tx = db_arc_clone.connection_pool.begin().await.unwrap();
        //                     db_arc_clone
        //                         .create_new_connection_event_by_client(
        //                             &mut tx,
        //                             &session.app_id,
        //                             &session.session_id,
        //                             &SessionType::Relay,
        //                             &ip_address,
        //                             None,
        //                             &(session_start + Duration::from_secs(1)),
        //                         )
        //                         .await
        //                         .unwrap();
        //                     tx.commit().await.unwrap();
        //                 }

        //                 db_arc_clone
        //                     .close_session(&session.session_id, session_end)
        //                     .await
        //                     .unwrap();
        //             }
        //         }
        //     });

        //     handles.push(handle);
        // }

        // // Wait for all tasks to complete
        // join_all(handles).await;

        // // Manually refresh the continuous aggregates
        // db_arc
        //     .refresh_continuous_aggregates(vec![
        //         "quarter_sessions_stats_per_app".to_string(),
        //         "hourly_sessions_stats_per_app".to_string(),
        //         "daily_sessions_stats_per_app".to_string(),
        //         // "monthly_sessions_stats_per_app".to_string(),
        //     ])
        //     .await
        //     .unwrap();

        // db_arc
        //     .refresh_continuous_aggregates(vec![
        //         "quarter_connection_stats_per_app".to_string(),
        //         "hourly_connection_stats_per_app".to_string(),
        //         "daily_connection_stats_per_app".to_string(),
        //         // "monthly_sessions_stats_per_app".to_string(),
        //     ])
        //     .await
        //     .unwrap();
    }

    #[tokio::test]
    async fn tesadasdasdasd() {
        let db = super::Db::connect_to_the_pool().await;
        // db.truncate_all_tables().await.unwrap();

        let privilege = UserAppPrivilege {
            user_id: "test_admin".to_string().clone(),
            app_id: "test_app_id_2".to_string().clone(),
            privilege_level: PrivilegeLevel::Read,
            creation_timestamp: to_microsecond_precision(&Utc::now()),
        };
        db.add_new_privilege(&privilege).await.unwrap();
    }

    #[tokio::test]
    async fn user_privs() {
        let db = super::Db::connect_to_the_pool().await;
        // db.truncate_all_tables().await.unwrap();

        let user_id = "test@gmail.com".to_string();
        let email = "test@gmail.com".to_string();

        db.add_new_user(&user_id, &email, None, None).await.unwrap();

        let privilege = UserAppPrivilege {
            user_id,
            app_id: "test_app_id_2".to_string().clone(),
            privilege_level: PrivilegeLevel::Read,
            creation_timestamp: to_microsecond_precision(&Utc::now()),
        };
        db.add_new_privilege(&privilege).await.unwrap();
    }

    #[tokio::test]
    async fn new_team() {
        let db = super::Db::connect_to_the_pool().await;
        // db.truncate_all_tables().await.unwrap();

        let team_id = "test_team_id_2".to_string();
        let app_id = "test_app_id_3".to_string();

        let user_id = "test_admin_2".to_string();
        let email = "admin_2".to_string();
        let password_hash = "pass_hash".to_string();

        let registration_timestamp = get_current_datetime();

        db.add_new_user(&user_id, &email, Some(&password_hash), None)
            .await
            .unwrap();

        let team = Team {
            team_id: team_id.clone(),
            team_name: "test_team_name".to_string(),
            personal: false,
            subscription: None,
            team_admin_id: user_id.clone(),
            registration_timestamp: registration_timestamp,
        };

        let registered_app = DbRegisteredApp {
            team_id: team_id.clone(),
            app_id: app_id.clone(),
            app_name: format!("{app_id}_APP_NAME").to_string(),
            whitelisted_domains: vec!["localhost".to_string()],
            ack_public_keys: vec!["key".to_string()],
            registration_timestamp: registration_timestamp,
        };

        let admin_privilege = UserAppPrivilege {
            app_id: app_id.clone(),
            creation_timestamp: registration_timestamp,
            privilege_level: PrivilegeLevel::Admin,
            user_id: user_id.clone(),
        };

        // Start a transaction
        let mut tx: Transaction<'_, sqlx::Postgres> = db.connection_pool.begin().await.unwrap();

        // Attempt to create the new team within the transaction
        let _create_team_result = db.create_new_team_within_tx(&mut tx, &team).await.unwrap();

        // Attempt to register the new app within the same transaction
        let _register_app_result = db
            .register_new_app_within_tx(&mut tx, &registered_app)
            .await
            .unwrap();

        // Attempt to add team admin within the same transaction
        let _add_admin_result = db
            .add_new_privilege_within_tx(&mut tx, &admin_privilege)
            .await
            .unwrap();

        // If both actions succeeded, commit the transaction
        tx.commit().await.unwrap();
    }

    #[tokio::test]
    async fn generate_session_stats_3() {
        let db = super::Db::connect_to_the_pool().await;

        let app_id_3 = "test_app_id_3".to_string();

        let now = get_current_datetime();
        let start_date = now - Duration::from_secs(60 * 60 * 24 * 60); // Start of second period, 60 days ago
        let end_date = now;
        let num_days = (end_date - start_date).num_days();

        let db_arc = Arc::new(db);

        let semaphore = Arc::new(Semaphore::new(8));
        let mut handles = vec![];

        for day_offset in 0..num_days {
            let app_id_clone = app_id_3.clone();
            let db_arc_clone = db_arc.clone();
            let sem_clone = semaphore.clone();

            let handle = tokio::task::spawn(async move {
                let _permit = sem_clone
                    .acquire()
                    .await
                    .expect("Failed to acquire semaphore permit");

                let day_start = start_date + Duration::from_secs(60 * 60 * 24 * day_offset as u64);

                let is_seventh_day = day_offset % 7 == 6; // day_offset is 0-based; 6 represents the 7th day

                // Loop through each hour of the day
                for hour in 0..24 {
                    let hour_start = day_start + Duration::from_secs(60 * 60 * hour as u64);
                    let session_count = if is_seventh_day {
                        // For the 7th day, limit sessions to 3-6 per hour
                        rand::thread_rng().gen_range(1..=5)
                    } else if hour >= 6 && hour <= 19 {
                        // Regular daytime hours
                        thread_rng().gen_range(40..=90)
                    } else {
                        // Regular nighttime hours
                        rand::thread_rng().gen_range(15..=45)
                    };

                    // Generate sessions for this hour
                    for i in 0..session_count {
                        let session_start =
                            hour_start + Duration::from_secs(rand::thread_rng().gen_range(0..3600));
                        let session_end = session_start + Duration::from_secs(600); // 10 minutes

                        let client_data = if i % 2 == 0 {
                            Some(ClientData {
                                client_id: "test_client_id".to_string(),
                                wallet_name: "test_wallet_name".to_string(),
                                wallet_type: "test_wallet_type".to_string(),
                                client_profile_id: 1,
                                connected_at: session_start,
                            })
                        } else {
                            None
                        };

                        let session = DbNcSession {
                            session_id: uuid7::uuid7().to_string(),
                            app_id: app_id_clone.clone(),
                            app_metadata: "test_metadata".to_string(),
                            persistent: false,
                            network: "Solana".to_string(),
                            client_data,
                            session_open_timestamp: session_start.clone(),
                            session_close_timestamp: Some(session_end),
                        };

                        db_arc_clone
                            .handle_new_session(
                                &session,
                                None,
                                &"127.0.0.1".to_string(),
                                &session_start,
                            )
                            .await
                            .unwrap();
                        db_arc_clone
                            .close_session(&session.session_id, session_end)
                            .await
                            .unwrap();
                    }
                }
            });

            handles.push(handle);
        }

        // Wait for all tasks to complete
        join_all(handles).await;

        // Manually refresh the continuous aggregates
        db_arc
            .refresh_continuous_aggregates(vec![
                "quarter_sessions_stats_per_app".to_string(),
                "hourly_sessions_stats_per_app".to_string(),
                "daily_sessions_stats_per_app".to_string(),
                // "monthly_sessions_stats_per_app".to_string(),
            ])
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn admin_privs_team_2() {
        let db = super::Db::connect_to_the_pool().await;
        // db.truncate_all_tables().await.unwrap();

        let privilege = UserAppPrivilege {
            user_id: "test_admin".to_string().clone(),
            app_id: "test_app_id_3".to_string().clone(),
            privilege_level: PrivilegeLevel::Read,
            creation_timestamp: to_microsecond_precision(&Utc::now()),
        };
        db.add_new_privilege(&privilege).await.unwrap();
    }
}
