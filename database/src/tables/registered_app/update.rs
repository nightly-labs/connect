use super::table_struct::{RegisteredApp, REGISTERED_APPS_KEYS, REGISTERED_APPS_TABLE_NAME};
use crate::{db::Db, structs::subscription::Subscription};
use sqlx::query;

impl Db {
    pub async fn register_new_app(&self, app: &RegisteredApp) -> Result<(), sqlx::Error> {
        let query_body = format!(
            "INSERT INTO {REGISTERED_APPS_TABLE_NAME} ({}) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
            REGISTERED_APPS_KEYS
        );

        let query_result = query(&query_body)
            .bind(&app.app_id)
            .bind(&app.app_name)
            .bind(&app.whitelisted_domains)
            .bind(&app.subscription)
            .bind(&app.ack_public_keys)
            .bind(&app.email)
            .bind(&(app.registration_timestamp as i64))
            .bind(&app.pass_hash)
            .execute(&self.connection_pool)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn update_subscription(
        &self,
        app_id: &String,
        subscription: &Subscription,
    ) -> Result<(), sqlx::Error> {
        let query_body = "UPDATE registered_apps SET subscription = $1 WHERE app_id = $2";
        let query_result = query(query_body)
            .bind(subscription)
            .bind(app_id)
            .execute(&self.connection_pool)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        structs::{
            consts::DAY_IN_SECONDS, request_status::RequestStatus, time_filters::TimeFilter,
        },
        tables::{
            registered_app::table_struct::RegisteredApp, requests::table_struct::Request,
            sessions::table_struct::DbNcSession,
        },
    };
    use sqlx::types::chrono::{DateTime, Utc};
    use std::{sync::Arc, time::Duration};
    use tokio::task;

    #[tokio::test]
    async fn test_requests_count() {
        let db = super::Db::connect_to_the_pool().await;
        db.truncate_all_tables().await.unwrap();

        // "Register" an app
        let app = RegisteredApp {
            app_id: "test_app_id".to_string(),
            app_name: "test_app_name".to_string(),
            whitelisted_domains: vec!["test_domain".to_string()],
            subscription: None,
            ack_public_keys: vec!["test_key".to_string()],
            email: None,
            registration_timestamp: 0,
            pass_hash: None,
        };

        db.register_new_app(&app).await.unwrap();

        let result = db.get_registered_app_by_app_id(&app.app_id).await.unwrap();
        assert_eq!(app, result);

        // Create session
        let session = DbNcSession {
            session_id: "test_session_id".to_string(),
            app_id: "test_app_id".to_string(),
            app_metadata: "test_app_metadata".to_string(),
            app_ip_address: "test_app_ip_address".to_string(),
            persistent: false,
            network: "test_network".to_string(),
            client: None,
            session_open_timestamp: DateTime::from(Utc::now()),
            session_close_timestamp: None,
        };

        db.save_new_session(&session).await.unwrap();

        let result = db.get_sessions_by_app_id(&app.app_id).await.unwrap();
        assert_eq!(result.len(), 1);
        // assert_eq!(session, result[0]);

        let db_arc = Arc::new(db);
        let mut tasks = Vec::new();

        for i in 0..33 {
            let db_clone = db_arc.clone(); // Clone the db connection or pool if needed
            tasks.push(task::spawn(async move {
                for j in 0..100 - i {
                    let creation_time: DateTime<Utc> = Utc::now()
                        - Duration::from_secs(i as u64 * DAY_IN_SECONDS as u64)
                        - Duration::from_millis((j + 1) as u64 * 100);

                    let request = Request {
                        request_id: format!("test_request_id_{}_{}", i, j),
                        app_id: "test_app_id".to_string(),
                        session_id: "test_session_id".to_string(),
                        network: "test_network".to_string(),
                        creation_timestamp: creation_time,
                        request_status: RequestStatus::Pending,
                        request_type: "test_request_type".to_string(),
                    };

                    if let Err(e) = db_clone.save_request(&request).await {
                        eprintln!("Failed to save request: {}", e);
                    }
                }
            }));
        }

        // Await all tasks to complete
        for task in tasks {
            task.await.unwrap();
        }

        // We need to refresh manually the views
        db_arc
            .refresh_continuous_aggregates(vec![
                "hourly_requests_stats".to_string(),
                "daily_requests_stats".to_string(),
                "monthly_requests_stats".to_string(),
            ])
            .await
            .unwrap();

        let result = db_arc
            .get_requests_stats_by_app_id(&app.app_id, TimeFilter::Last24Hours)
            .await
            .unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].request_count, 100);
        assert_eq!(result[1].request_count, 99);

        let result = db_arc
            .get_requests_stats_by_app_id(&app.app_id, TimeFilter::Last7Days)
            .await
            .unwrap();

        assert_eq!(result.len(), 8);
        assert_eq!(result[0].request_count, 100);
        assert_eq!(result[7].request_count, 93);

        let result = db_arc
            .get_requests_stats_by_app_id(&app.app_id, TimeFilter::Last30Days)
            .await
            .unwrap();

        assert_eq!(result.len(), 31);
        assert_eq!(result[0].request_count, 100);
        assert_eq!(result[30].request_count, 70);
    }

    #[tokio::test]
    async fn test_requests_success_rate() {
        let db = super::Db::connect_to_the_pool().await;
        db.truncate_all_tables().await.unwrap();

        // "Register" an app
        let app = RegisteredApp {
            app_id: "test_app_id".to_string(),
            app_name: "test_app_name".to_string(),
            whitelisted_domains: vec!["test_domain".to_string()],
            subscription: None,
            ack_public_keys: vec!["test_key".to_string()],
            email: None,
            registration_timestamp: 0,
            pass_hash: None,
        };

        db.register_new_app(&app).await.unwrap();

        let result = db.get_registered_app_by_app_id(&app.app_id).await.unwrap();
        assert_eq!(app, result);

        // Create session
        let session = DbNcSession {
            session_id: "test_session_id".to_string(),
            app_id: "test_app_id".to_string(),
            app_metadata: "test_app_metadata".to_string(),
            app_ip_address: "test_app_ip_address".to_string(),
            persistent: false,
            network: "test_network".to_string(),
            client: None,
            session_open_timestamp: DateTime::from(Utc::now()),
            session_close_timestamp: None,
        };

        db.save_new_session(&session).await.unwrap();

        let result = db.get_sessions_by_app_id(&app.app_id).await.unwrap();
        assert_eq!(result.len(), 1);
        // assert_eq!(session, result[0]);

        let db_arc = Arc::new(db);
        let mut tasks = Vec::new();

        for i in 0..33 {
            let db_clone = db_arc.clone(); // Clone the db connection or pool if needed
            tasks.push(task::spawn(async move {
                for j in 0..100 - i {
                    let creation_time: DateTime<Utc> = Utc::now()
                        - Duration::from_secs(i as u64 * DAY_IN_SECONDS as u64)
                        - Duration::from_millis((j + 1) as u64 * 100);

                    let status = if j % 3 == 0 {
                        RequestStatus::Completed
                    } else if j % 3 == 1 {
                        RequestStatus::Completed
                    } else {
                        RequestStatus::Rejected
                    };

                    let request = Request {
                        request_id: format!("test_request_id_{}_{}", i, j),
                        app_id: "test_app_id".to_string(),
                        session_id: "test_session_id".to_string(),
                        network: "test_network".to_string(),
                        creation_timestamp: creation_time,
                        request_status: status,
                        request_type: "test_request_type".to_string(),
                    };
                    if let Err(e) = db_clone.save_request(&request).await {
                        eprintln!("Failed to save request: {}", e);
                    }
                }
            }));
        }

        // Await all tasks to complete
        for task in tasks {
            task.await.unwrap();
        }

        // We need to refresh manually the views
        db_arc
            .refresh_continuous_aggregates(vec![
                "hourly_requests_stats".to_string(),
                "daily_requests_stats".to_string(),
                "monthly_requests_stats".to_string(),
            ])
            .await
            .unwrap();

        // Check the success rate on every time filter
        let result = db_arc
            .get_requests_stats_by_app_id(&app.app_id, TimeFilter::Last24Hours)
            .await
            .unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(
            (result[0].success_rate.unwrap() * 100.0).ceil() / 100.0,
            0.67 as f64
        );
        assert_eq!(
            (result[1].success_rate.unwrap() * 100.0).ceil() / 100.0,
            0.67 as f64
        );

        let result = db_arc
            .get_requests_stats_by_app_id(&app.app_id, TimeFilter::Last7Days)
            .await
            .unwrap();

        assert_eq!(result.len(), 8);
        assert_eq!(
            (result[0].success_rate.unwrap() * 100.0).ceil() / 100.0,
            0.67 as f64
        );
        assert_eq!(
            (result[7].success_rate.unwrap() * 100.0).ceil() / 100.0,
            0.67 as f64
        );

        let result = db_arc
            .get_requests_stats_by_app_id(&app.app_id, TimeFilter::Last30Days)
            .await
            .unwrap();

        assert_eq!(result.len(), 31);
        assert_eq!(
            (result[0].success_rate.unwrap() * 100.0).ceil() / 100.0,
            0.67 as f64
        );
        assert_eq!(
            (result[30].success_rate.unwrap() * 100.0).ceil() / 100.0,
            0.68 as f64
        );

        // Test missing success due to all requests having pending status
        // Add new app to have a "clean" state
        let app = RegisteredApp {
            app_id: "test_app_id2".to_string(),
            app_name: "test_app_name".to_string(),
            whitelisted_domains: vec!["test_domain".to_string()],
            subscription: None,
            ack_public_keys: vec!["test_key".to_string()],
            email: None,
            registration_timestamp: 0,
            pass_hash: None,
        };

        db_arc.register_new_app(&app).await.unwrap();

        let result = db_arc
            .get_registered_app_by_app_id(&app.app_id)
            .await
            .unwrap();
        assert_eq!(app, result);

        // Create session
        let session = DbNcSession {
            session_id: "test_session_id".to_string(),
            app_id: "test_app_id2".to_string(),
            app_metadata: "test_app_metadata".to_string(),
            app_ip_address: "test_app_ip_address".to_string(),
            persistent: false,
            network: "test_network".to_string(),
            client: None,
            session_open_timestamp: DateTime::from(Utc::now()),
            session_close_timestamp: None,
        };

        db_arc.save_new_session(&session).await.unwrap();

        let mut tasks = Vec::new();
        for i in 0..10 {
            let db_clone = db_arc.clone(); // Clone the db connection or pool if needed
            tasks.push(task::spawn(async move {
                for j in 0..11 - i {
                    let creation_time: DateTime<Utc> = Utc::now()
                        - Duration::from_secs(i as u64 * DAY_IN_SECONDS as u64)
                        - Duration::from_millis((j + 1) as u64 * 100);

                    let request = Request {
                        request_id: format!("test_request_id_{}_{}", i, j),
                        app_id: "test_app_id2".to_string(),
                        session_id: "test_session_id".to_string(),
                        network: "test_network".to_string(),
                        creation_timestamp: creation_time,
                        request_status: RequestStatus::Pending,
                        request_type: "test_request_type".to_string(),
                    };
                    if let Err(e) = db_clone.save_request(&request).await {
                        eprintln!("Failed to save request: {}", e);
                    }
                }
            }));
        }

        // Await all tasks to complete
        for task in tasks {
            task.await.unwrap();
        }

        // We need to refresh manually the views
        db_arc
            .refresh_continuous_aggregates(vec![
                "hourly_requests_stats".to_string(),
                "daily_requests_stats".to_string(),
                "monthly_requests_stats".to_string(),
            ])
            .await
            .unwrap();

        let result = db_arc
            .get_requests_stats_by_app_id(&app.app_id, TimeFilter::Last24Hours)
            .await
            .unwrap();

        assert_eq!(result.len(), 2);
        assert!(result[0].success_rate.is_none());
        assert!(result[1].success_rate.is_none());

        let result = db_arc
            .get_requests_stats_by_app_id(&app.app_id, TimeFilter::Last7Days)
            .await
            .unwrap();

        assert_eq!(result.len(), 8);
        assert!(result[0].success_rate.is_none());
        assert!(result[7].success_rate.is_none());

        let result = db_arc
            .get_requests_stats_by_app_id(&app.app_id, TimeFilter::Last30Days)
            .await
            .unwrap();

        assert_eq!(result.len(), 10);
        assert!(result[0].success_rate.is_none());
        assert!(result[9].success_rate.is_none());
    }

    #[tokio::test]
    async fn test_average_session_duration() {
        let db = super::Db::connect_to_the_pool().await;
        db.truncate_all_tables().await.unwrap();

        // "Register" an app
        let app = RegisteredApp {
            app_id: "test_app_id".to_string(),
            app_name: "test_app_name".to_string(),
            whitelisted_domains: vec!["test_domain".to_string()],
            subscription: None,
            ack_public_keys: vec!["test_key".to_string()],
            email: None,
            registration_timestamp: 0,
            pass_hash: None,
        };

        db.register_new_app(&app).await.unwrap();

        let result = db.get_registered_app_by_app_id(&app.app_id).await.unwrap();
        assert_eq!(app, result);

        // Create sessions
        let now = Utc::now();
        let sessions = vec![
            (
                now - Duration::from_secs(4 * 60 * 60),
                now - Duration::from_secs((4 * 60 - 85) * 60),
            ), // 1 hour 25 minutes session, 4 hours ago
            (
                now - Duration::from_secs((2 * 60 + 35) * 60),
                now - Duration::from_secs((2 * 60 + 10) * 60),
            ), // 25 minutes session, 2 hours and 35 minutes ago
            (
                now - Duration::from_secs((1 * 60 + 50) * 60),
                now - Duration::from_secs((1 * 60 + 40) * 60),
            ), // 10 minutes session, 1 hour and 50 minutes ago
        ];

        for (start, end) in sessions.iter() {
            let session = DbNcSession {
                session_id: format!("session_id_{}", start.timestamp()),
                app_id: "test_app_id".to_string(),
                app_metadata: "test_app_metadata".to_string(),
                app_ip_address: "test_app_ip_address".to_string(),
                persistent: false,
                network: "test_network".to_string(),
                client: None,
                session_open_timestamp: *start,
                session_close_timestamp: None,
            };

            db.save_new_session(&session).await.unwrap();
            db.close_session(&session.session_id, *end).await.unwrap();
        }

        // We need to refresh manually the views
        db.refresh_continuous_aggregates(vec![
            "sessions_stats_per_app_monthly".to_string(),
            "sessions_stats_per_app_daily".to_string(),
        ])
        .await
        .unwrap();

        let result = db.get_monthly_sessions_stats(&app.app_id).await.unwrap();

        assert_eq!(result.len(), 1);

        let expected_avg_duration_seconds: f64 = sessions
            .iter()
            .map(|(start, end)| (end.timestamp() - start.timestamp()) as f64)
            .sum::<f64>()
            / sessions.len() as f64;

        assert_eq!(
            result[0].average_duration_seconds,
            expected_avg_duration_seconds
        );
    }

    #[tokio::test]
    async fn test_sessions_count() {
        let db = super::Db::connect_to_the_pool().await;
        db.truncate_all_tables().await.unwrap();

        // "Register" an app
        let app = RegisteredApp {
            app_id: "test_app_id".to_string(),
            app_name: "test_app_name".to_string(),
            whitelisted_domains: vec!["test_domain".to_string()],
            subscription: None,
            ack_public_keys: vec!["test_key".to_string()],
            email: None,
            registration_timestamp: 0,
            pass_hash: None,
        };

        db.register_new_app(&app).await.unwrap();

        let result = db.get_registered_app_by_app_id(&app.app_id).await.unwrap();
        assert_eq!(app, result);

        // Number of sessions to create
        let num_sessions: u64 = 100;
        let now = Utc::now();
        let start_of_period = now - Duration::from_secs(60 * 60 * 24 * 14); // 14 days

        // Generate and save sessions
        for i in 0..num_sessions {
            let session_start =
                start_of_period + Duration::from_secs(i * 86400 / num_sessions as u64); // spread sessions evenly over 14 days
            let session_end = session_start + Duration::from_secs(60 * 30); // duration of 30 minutes for each session

            let session = DbNcSession {
                session_id: format!("session_{}_{}", app.app_id, i),
                app_id: app.app_id.clone(),
                app_metadata: "test_metadata".to_string(),
                app_ip_address: "127.0.0.1".to_string(),
                persistent: false,
                network: "test_network".to_string(),
                client: None,
                session_open_timestamp: session_start,
                session_close_timestamp: None,
            };

            db.save_new_session(&session).await.unwrap();
            db.close_session(&session.session_id, session_end)
                .await
                .unwrap();
        }

        // Manually refresh the continuous aggregates
        db.refresh_continuous_aggregates(vec![
            "sessions_stats_per_app_monthly".to_string(),
            "sessions_stats_per_app_daily".to_string(),
        ])
        .await
        .unwrap();

        let stats = db.get_monthly_sessions_stats(&app.app_id).await.unwrap();

        assert_eq!(stats.len(), 1);
        assert_eq!(stats[0].sessions_opened, num_sessions as i64);
    }

    #[tokio::test]
    async fn test_sessions_average_daily_opened_sessions() {
        let db = super::Db::connect_to_the_pool().await;
        db.truncate_all_tables().await.unwrap();

        // "Register" an app
        let app = RegisteredApp {
            app_id: "test_app_id".to_string(),
            app_name: "test_app_name".to_string(),
            whitelisted_domains: vec!["test_domain".to_string()],
            subscription: None,
            ack_public_keys: vec!["test_key".to_string()],
            email: None,
            registration_timestamp: 0,
            pass_hash: None,
        };

        db.register_new_app(&app).await.unwrap();

        let result = db.get_registered_app_by_app_id(&app.app_id).await.unwrap();
        assert_eq!(app, result);

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
                session_id: format!("session_{}_{}", app.app_id, i),
                app_id: app.app_id.clone(),
                app_metadata: "test_metadata".to_string(),
                app_ip_address: "127.0.0.1".to_string(),
                persistent: false,
                network: "test_network".to_string(),
                client: None,
                session_open_timestamp: session_start,
                session_close_timestamp: Some(session_end),
            };

            db.save_new_session(&session).await.unwrap();
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
                session_id: format!("session_{}_{}_2nd", app.app_id, i), // Ensure unique session IDs for the second period
                app_id: app.app_id.clone(),
                app_metadata: "test_metadata".to_string(),
                app_ip_address: "127.0.0.1".to_string(),
                persistent: false,
                network: "test_network".to_string(),
                client: None,
                session_open_timestamp: session_start,
                session_close_timestamp: Some(session_end),
            };

            db.save_new_session(&session).await.unwrap();
            db.close_session(&session.session_id, session_end)
                .await
                .unwrap();
        }

        // Manually refresh the continuous aggregates
        db.refresh_continuous_aggregates(vec![
            "sessions_stats_per_app_daily".to_string(),
            "sessions_stats_per_app_monthly".to_string(),
        ])
        .await
        .unwrap();

        let stats = db.get_monthly_sessions_stats(&app.app_id).await.unwrap();

        // assert_eq!(stats.len(), 2);
        println!("Stats LEN: {:?}", stats.len());
        for stat in stats {
            println!("Stat: {:?}", stat);
        }
    }
}
