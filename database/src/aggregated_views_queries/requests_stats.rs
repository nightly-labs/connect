use crate::{
    db::Db,
    structs::{filter_requests::RequestsStats, time_filters::TimeFilter},
    tables::utils::{format_view_keys, format_view_name},
};
use sqlx::Error;

pub const REQUESTS_STATS_BASE_VIEW_NAME: &str = "requests_stats_per_app";
pub const REQUESTS_STATS_BASE_KEYS: [(&'static str, bool); 4] = [
    ("app_id", false),
    ("bucket", true),
    ("request_count", true),
    ("success_rate", true),
];

impl Db {
    pub async fn get_requests_stats_by_app_id(
        &self,
        app_id: &str,
        filter: TimeFilter,
    ) -> Result<Vec<RequestsStats>, Error> {
        let start_date = filter.to_date();
        let bucket_size = filter.bucket_size();

        // Correctly selecting the view based on the bucket_size
        let prefix = match bucket_size {
            "1 hour" => "hourly",
            "1 day" => "daily",
            "1 month" => "monthly",
            // TODO for now return WorkerCrashed but later create custom error
            _ => return Err(Error::WorkerCrashed),
        };

        let formatted_keys = format_view_keys(prefix, &REQUESTS_STATS_BASE_KEYS);
        let formatted_view_name = format_view_name(prefix, REQUESTS_STATS_BASE_VIEW_NAME);
        let filter_key = REQUESTS_STATS_BASE_KEYS[1].0;
        let filter = format!("{prefix}_{filter_key}");

        let query = format!(
            "SELECT {formatted_keys}
            FROM {formatted_view_name}
            WHERE app_id = $1 AND {filter} >= $2
            ORDER BY {filter} DESC",
        );

        sqlx::query_as::<_, RequestsStats>(&query)
            .bind(app_id)
            .bind(start_date)
            .fetch_all(&self.connection_pool)
            .await
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use crate::{
        structs::{
            consts::DAY_IN_SECONDS, request_status::RequestStatus, session_type::SessionType,
        },
        tables::{
            registered_app::table_struct::DbRegisteredApp, requests::table_struct::Request,
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

        // Create test team instance
        let team_id = "test_team_id".to_string();
        let app_id = "test_app_id".to_string();

        db.setup_test_team(&team_id, &app_id, Utc::now())
            .await
            .unwrap();

        // Create session
        let session = DbNcSession {
            session_id: "test_session_id".to_string(),
            app_id: app_id.to_string(),
            session_type: SessionType::Relay,
            app_metadata: "test_app_metadata".to_string(),
            app_ip_address: "test_app_ip_address".to_string(),
            persistent: false,
            network: "test_network".to_string(),
            client_profile_id: None,
            client: None,
            session_open_timestamp: DateTime::from(Utc::now()),
            session_close_timestamp: None,
        };

        db.handle_new_session(&session, &"connection_id".to_string())
            .await
            .unwrap();

        let result = db.get_sessions_by_app_id(&app_id).await.unwrap();
        assert_eq!(result.len(), 1);

        let db_arc = Arc::new(db);
        let mut tasks = Vec::new();

        for i in 0..33 {
            let db_clone = db_arc.clone(); // Clone the db connection or pool if needed
            let app_id = app_id.clone();
            tasks.push(task::spawn(async move {
                for j in 0..100 - i {
                    let creation_time: DateTime<Utc> = Utc::now()
                        - Duration::from_secs(i as u64 * DAY_IN_SECONDS as u64)
                        - Duration::from_millis((j + 1) as u64 * 100);

                    let request = Request {
                        request_id: format!("test_request_id_{}_{}", i, j),
                        app_id: app_id.to_string(),
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
                "hourly_requests_stats_per_app".to_string(),
                "daily_requests_stats_per_app".to_string(),
                "monthly_requests_stats_per_app".to_string(),
            ])
            .await
            .unwrap();

        let result = db_arc
            .get_requests_stats_by_app_id(&app_id, TimeFilter::Last24Hours)
            .await
            .unwrap();

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].request_count, 100);
        assert_eq!(result[1].request_count, 99);

        let result = db_arc
            .get_requests_stats_by_app_id(&app_id, TimeFilter::Last7Days)
            .await
            .unwrap();

        assert_eq!(result.len(), 8);
        assert_eq!(result[0].request_count, 100);
        assert_eq!(result[7].request_count, 93);

        let result = db_arc
            .get_requests_stats_by_app_id(&app_id, TimeFilter::Last30Days)
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

        // Create test team instance
        let team_id = "test_team_id".to_string();
        let app_id = "test_app_id".to_string();

        db.setup_test_team(&team_id, &app_id, Utc::now())
            .await
            .unwrap();

        // Create session
        let session = DbNcSession {
            session_id: "test_session_id".to_string(),
            app_id: "test_app_id".to_string(),
            session_type: SessionType::Relay,
            app_metadata: "test_app_metadata".to_string(),
            app_ip_address: "test_app_ip_address".to_string(),
            persistent: false,
            network: "test_network".to_string(),
            client_profile_id: None,
            client: None,
            session_open_timestamp: DateTime::from(Utc::now()),
            session_close_timestamp: None,
        };

        db.handle_new_session(&session, &"connection_id".to_string())
            .await
            .unwrap();

        let result = db.get_sessions_by_app_id(&app_id).await.unwrap();
        assert_eq!(result.len(), 1);
        // assert_eq!(session, result[0]);

        let db_arc = Arc::new(db);
        let mut tasks = Vec::new();

        for i in 0..33 {
            let db_clone = db_arc.clone(); // Clone the db connection or pool if needed
            let app_id = app_id.clone();
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
                        app_id: app_id.to_string(),
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
                "hourly_requests_stats_per_app".to_string(),
                "daily_requests_stats_per_app".to_string(),
                "monthly_requests_stats_per_app".to_string(),
            ])
            .await
            .unwrap();

        // Check the success rate on every time filter
        let result = db_arc
            .get_requests_stats_by_app_id(&app_id, TimeFilter::Last24Hours)
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
            .get_requests_stats_by_app_id(&app_id, TimeFilter::Last7Days)
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
            .get_requests_stats_by_app_id(&app_id, TimeFilter::Last30Days)
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
        let second_app_id = "test_app_id2".to_string();
        let app = DbRegisteredApp {
            team_id: team_id.clone(),
            app_id: second_app_id.to_string(),
            app_name: "test_app_name".to_string(),
            whitelisted_domains: vec!["test_domain".to_string()],
            subscription: None,
            ack_public_keys: vec!["test_key".to_string()],
            email: None,
            registration_timestamp: Utc::now(),
            pass_hash: None,
        };
        db_arc.register_new_app(&app).await.unwrap();

        let result = db_arc
            .get_registered_app_by_app_id(&second_app_id)
            .await
            .unwrap();
        assert_eq!(app, result);

        // Create session
        let session = DbNcSession {
            session_id: "test_session_id".to_string(),
            app_id: second_app_id.to_string(),
            session_type: SessionType::Relay,
            app_metadata: "test_app_metadata".to_string(),
            app_ip_address: "test_app_ip_address".to_string(),
            persistent: false,
            network: "test_network".to_string(),
            client_profile_id: None,
            client: None,
            session_open_timestamp: DateTime::from(Utc::now()),
            session_close_timestamp: None,
        };

        db_arc
            .handle_new_session(&session, &"connection_id".to_string())
            .await
            .unwrap();

        let mut tasks = Vec::new();
        for i in 0..10 {
            let db_clone = db_arc.clone(); // Clone the db connection or pool if needed
            let app_id = second_app_id.clone();
            tasks.push(task::spawn(async move {
                for j in 0..11 - i {
                    let creation_time: DateTime<Utc> = Utc::now()
                        - Duration::from_secs(i as u64 * DAY_IN_SECONDS as u64)
                        - Duration::from_millis((j + 1) as u64 * 100);

                    let request = Request {
                        request_id: format!("test_request_id_{}_{}", i, j),
                        app_id: app_id.to_string(),
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
                "hourly_requests_stats_per_app".to_string(),
                "daily_requests_stats_per_app".to_string(),
                "monthly_requests_stats_per_app".to_string(),
            ])
            .await
            .unwrap();

        let result = db_arc
            .get_requests_stats_by_app_id(&second_app_id, TimeFilter::Last24Hours)
            .await
            .unwrap();

        assert_eq!(result.len(), 2);
        assert!(result[0].success_rate.is_none());
        assert!(result[1].success_rate.is_none());

        let result = db_arc
            .get_requests_stats_by_app_id(&second_app_id, TimeFilter::Last7Days)
            .await
            .unwrap();

        assert_eq!(result.len(), 8);
        assert!(result[0].success_rate.is_none());
        assert!(result[7].success_rate.is_none());

        let result = db_arc
            .get_requests_stats_by_app_id(&second_app_id, TimeFilter::Last30Days)
            .await
            .unwrap();

        assert_eq!(result.len(), 10);
        assert!(result[0].success_rate.is_none());
        assert!(result[9].success_rate.is_none());
    }
}
