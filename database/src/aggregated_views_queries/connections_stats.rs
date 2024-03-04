use crate::{
    db::Db,
    structs::{filter_requests::RequestsStats, time_filters::TimeFilter},
    tables::utils::{format_view_keys, format_view_name},
};
use sqlx::Error;

pub const CONNECTIONS_STATS_BASE_VIEW_NAME: &str = "connection_stats_per_app_and_network";
pub const CONNECTIONS_STATS_BASE_KEYS: [(&'static str, bool); 5] = [
    ("app_id", false),
    ("network", false),
    ("bucket", true),
    ("app_connection_count", true),
    ("client_connection_count", true),
];

impl Db {
    pub async fn get_connections_stats_by_app_id(
        &self,
        app_id: &str,
        network: Option<&str>,
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

        let formatted_keys = format_view_keys(prefix, &CONNECTIONS_STATS_BASE_KEYS);
        let formatted_view_name = format_view_name(prefix, CONNECTIONS_STATS_BASE_VIEW_NAME);
        let date_filter_key = CONNECTIONS_STATS_BASE_KEYS[1].0;
        let filter = format!("{prefix}_{date_filter_key}");

        let network_filter = match network {
            Some(network) => format!("AND network = '{network}'"),
            None => "".to_string(),
        };

        let query = format!(
            "SELECT {formatted_keys}
            FROM {formatted_view_name}
            WHERE app_id = $1 AND {filter} >= $2 {network_filter}
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
mod tests {
    use std::process::Command;

    use crate::{structs::time_filters::TimeFilter, tables::sessions::table_struct::DbNcSession};
    use sqlx::types::chrono::Utc;

    #[tokio::test]
    async fn test_connections_all_networks() {
        let db = super::Db::connect_to_the_pool().await;
        db.truncate_all_tables().await.unwrap();

        // Create test team instance
        let team_id = "test_team_id".to_string();
        let app_id = "test_app_id".to_string();

        db.setup_test_team(&team_id, &app_id, Utc::now())
            .await
            .unwrap();

        let networks = vec![
            "test_network_1",
            "test_network_2",
            "test_network_3",
            "test_network_4",
            "test_network_5",
        ];
        // Create persistent a session for each odd number of network, for each session connect via app 3 times and for client connect number of network times
        for (i, network) in networks.iter().enumerate() {
            let session = DbNcSession {
                session_id: format!("session_{app_id}_{i}"),
                app_id: app_id.clone(),
                app_metadata: "test_metadata".to_string(),
                app_ip_address: "".to_string(),
                persistent: true,
                network: network.to_string(),
                client_profile_id: None,
                client: None,
                session_open_timestamp: Utc::now(),
                session_close_timestamp: None,
            };

            db.handle_new_session(&session, &format!("connection_id_{app_id}_{i}").to_string())
                .await
                .unwrap();
        }

        // // Number of sessions to create
        // let num_sessions: u64 = 100;
        // let now = Utc::now();
        // let start_of_period = now - Duration::from_secs(60 * 60 * 24 * 14); // 14 days

        // // Generate and save sessions
        // for i in 0..num_sessions {
        //     let session_start =
        //         start_of_period + Duration::from_secs(i * 86400 / num_sessions as u64); // spread sessions evenly over 14 days
        //     let session_end = session_start + Duration::from_secs(60 * 30); // duration of 30 minutes for each session

        //     let session = DbNcSession {
        //         session_id: format!("session_{}_{}", app_id, i),
        //         app_id: app_id.clone(),
        //         app_metadata: "test_metadata".to_string(),
        //         app_ip_address: "127.0.0.1".to_string(),
        //         persistent: false,
        //         network: "test_network".to_string(),
        //         client_profile_id: None,
        //         client: None,
        //         session_open_timestamp: session_start,
        //         session_close_timestamp: None,
        //     };

        //     db.handle_new_session(&session, &"connection_id".to_string())
        //         .await
        //         .unwrap();
        //     db.close_session(&session.session_id, session_end)
        //         .await
        //         .unwrap();
        // }

        // Manually refresh the continuous aggregates
        db.refresh_continuous_aggregates(vec![
            "hourly_connection_stats_per_app_and_network".to_string(),
            "daily_connection_stats_per_app_and_network".to_string(),
            "monthly_connection_stats_per_app_and_network".to_string(),
        ])
        .await
        .unwrap();

        let stats = db
            .get_connections_stats_by_app_id(&app_id, None, TimeFilter::LastMonth)
            .await
            .unwrap();

        println!("{:?}", stats);
        // assert_eq!(stats.len(), 1);
        // assert_eq!(stats[0].sessions_opened, num_sessions as i64);
    }

    // #[tokio::test]
    // async fn test_sessions_average_daily_sessions_opened() {
    //     let db = super::Db::connect_to_the_pool().await;
    //     db.truncate_all_tables().await.unwrap();

    //     // Create test team instance
    //     let team_id = "test_team_id".to_string();
    //     let app_id = "test_app_id".to_string();

    //     db.setup_test_team(&team_id, &app_id, Utc::now())
    //         .await
    //         .unwrap();

    //     let now = Utc::now();
    //     let start_of_first_period = now - Duration::from_secs(60 * 60 * 24 * 60); // Start of first period, 60 days ago
    //     let start_of_second_period = now - Duration::from_secs(60 * 60 * 24 * 30); // Start of second period, 30 days ago
    //     let num_sessions_first_period: u64 = 40;
    //     let num_sessions_second_period: u64 = 28;

    //     // Generate and save sessions for the first period
    //     for i in 0..num_sessions_first_period {
    //         let session_start = start_of_first_period
    //             + Duration::from_secs(i * 86400 / num_sessions_first_period as u64);
    //         let session_end = session_start + Duration::from_secs(60 * 30); // Duration of 30 minutes for each session

    //         let session = DbNcSession {
    //             session_id: format!("session_{}_{}", app_id, i),
    //             app_id: app_id.clone(),
    //             app_metadata: "test_metadata".to_string(),
    //             app_ip_address: "127.0.0.1".to_string(),
    //             persistent: false,
    //             network: "test_network".to_string(),
    //             client_profile_id: None,
    //             client: None,
    //             session_open_timestamp: session_start,
    //             session_close_timestamp: Some(session_end),
    //         };

    //         db.handle_new_session(&session, &"connection_id".to_string())
    //             .await
    //             .unwrap();
    //         db.close_session(&session.session_id, session_end)
    //             .await
    //             .unwrap();
    //     }

    //     // Generate and save sessions for the second period
    //     for i in 0..num_sessions_second_period {
    //         let session_start = start_of_second_period
    //             + Duration::from_secs(i * 86400 / num_sessions_second_period as u64);
    //         let session_end = session_start + Duration::from_secs(60 * 30); // Duration of 30 minutes for each session

    //         let session = DbNcSession {
    //             session_id: format!("session_{}_{}_2nd", app_id, i), // Ensure unique session IDs for the second period
    //             app_id: app_id.clone(),
    //             app_metadata: "test_metadata".to_string(),
    //             app_ip_address: "127.0.0.1".to_string(),
    //             persistent: false,
    //             network: "test_network".to_string(),
    //             client_profile_id: None,
    //             client: None,
    //             session_open_timestamp: session_start,
    //             session_close_timestamp: Some(session_end),
    //         };

    //         db.handle_new_session(&session, &"connection_id".to_string())
    //             .await
    //             .unwrap();
    //         db.close_session(&session.session_id, session_end)
    //             .await
    //             .unwrap();
    //     }

    //     // Manually refresh the continuous aggregates
    //     db.refresh_continuous_aggregates(vec![
    //         "hourly_sessions_stats_per_app".to_string(),
    //         "daily_sessions_stats_per_app".to_string(),
    //         "monthly_sessions_stats_per_app".to_string(),
    //     ])
    //     .await
    //     .unwrap();

    //     let stats = db
    //         .get_sessions_stats_by_app_id(&app_id, TimeFilter::Last30Days)
    //         .await
    //         .unwrap();

    //     assert_eq!(stats.len(), 2);
    // }
}
