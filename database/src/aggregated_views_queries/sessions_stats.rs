use crate::{
    db::Db,
    structs::{filter_requests::SessionsStats, time_filters::TimeFilter},
    tables::utils::{format_view_keys, format_view_name},
};
use sqlx::Error;

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
    ) -> Result<Vec<SessionsStats>, Error> {
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
    }
}

#[cfg(test)]
mod tests {
    use crate::{structs::time_filters::TimeFilter, tables::sessions::table_struct::DbNcSession};
    use sqlx::types::chrono::Utc;
    use std::time::Duration;

    #[tokio::test]
    async fn test_sessions_count() {
        let db = super::Db::connect_to_the_pool().await;
        db.truncate_all_tables().await.unwrap();

        // Create test team instance
        let team_id = "test_team_id".to_string();
        let app_id = "test_app_id".to_string();

        db.setup_test_team(&team_id, &app_id, Utc::now())
            .await
            .unwrap();

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
                session_id: format!("session_{}_{}", app_id, i),
                app_id: app_id.clone(),
                app_metadata: "test_metadata".to_string(),
                app_ip_address: "127.0.0.1".to_string(),
                persistent: false,
                network: "test_network".to_string(),
                client_profile_id: None,
                client: None,
                session_open_timestamp: session_start,
                session_close_timestamp: None,
            };

            db.handle_new_session(&session, &"connection_id".to_string())
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
            "monthly_sessions_stats_per_app".to_string(),
        ])
        .await
        .unwrap();

        let stats = db
            .get_sessions_stats_by_app_id(&app_id, TimeFilter::LastMonth)
            .await
            .unwrap();

        println!("{:?}", stats);
        // assert_eq!(stats.len(), 1);
        // assert_eq!(stats[0].sessions_opened, num_sessions as i64);
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
                app_ip_address: "127.0.0.1".to_string(),
                persistent: false,
                network: "test_network".to_string(),
                client_profile_id: None,
                client: None,
                session_open_timestamp: session_start,
                session_close_timestamp: Some(session_end),
            };

            db.handle_new_session(&session, &"connection_id".to_string())
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
                app_ip_address: "127.0.0.1".to_string(),
                persistent: false,
                network: "test_network".to_string(),
                client_profile_id: None,
                client: None,
                session_open_timestamp: session_start,
                session_close_timestamp: Some(session_end),
            };

            db.handle_new_session(&session, &"connection_id".to_string())
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
            "monthly_sessions_stats_per_app".to_string(),
        ])
        .await
        .unwrap();

        let stats = db
            .get_sessions_stats_by_app_id(&app_id, TimeFilter::Last30Days)
            .await
            .unwrap();

        assert_eq!(stats.len(), 2);
    }
}
