use crate::{
    db::Db,
    structs::{db_error::DbError, filter_requests::ConnectionStats, time_filters::TimeFilter},
    tables::utils::{format_view_keys, format_view_name},
};

pub const CONNECTIONS_STATS_BASE_VIEW_NAME: &str = "connection_stats_per_app_and_network";
pub const CONNECTIONS_STATS_BASE_KEYS: [(&'static str, bool); 5] = [
    ("app_id", false),
    ("network", false),
    ("bucket", true),
    ("app_connection_count", true),
    ("clients_connection_count", true),
];

impl Db {
    pub async fn get_connections_stats_by_app_id(
        &self,
        app_id: &str,
        network: Option<&str>,
        filter: TimeFilter,
    ) -> Result<Vec<ConnectionStats>, DbError> {
        let start_date = filter.to_date();
        let bucket_size = filter.bucket_size();

        // Correctly selecting the view based on the bucket_size
        let prefix = match bucket_size {
            "1 hour" => "hourly",
            "1 day" => "daily",
            "1 month" => "monthly",
            _ => return Err(DbError::DatabaseError("Invalid bucket size".to_string())),
        };

        let formatted_keys = format_view_keys(prefix, &CONNECTIONS_STATS_BASE_KEYS);
        let formatted_view_name = format_view_name(prefix, CONNECTIONS_STATS_BASE_VIEW_NAME);
        let date_filter_key = CONNECTIONS_STATS_BASE_KEYS[2].0;
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

        sqlx::query_as::<_, ConnectionStats>(&query)
            .bind(app_id)
            .bind(start_date)
            .fetch_all(&self.connection_pool)
            .await
            .map_err(|e| e.into())
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        structs::{session_type::SessionType, time_filters::TimeFilter},
        tables::sessions::table_struct::DbNcSession,
    };
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
            let session_id = format!("session_{app_id}_{i}");

            let session = DbNcSession {
                session_id: session_id.clone(),
                app_id: app_id.clone(),
                session_type: SessionType::Relay,
                app_metadata: "test_metadata".to_string(),
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

            // Each time a session is created, means that app has been connected, create 2 more connections
            let mut tx = db.connection_pool.begin().await.unwrap();
            db.create_new_connection_event_by_app(
                &mut tx,
                &session_id,
                &format!("connection_id_{app_id}_{i}_1").to_string(),
                &app_id,
                &network.to_string(),
            )
            .await
            .unwrap();

            db.create_new_connection_event_by_app(
                &mut tx,
                &session_id,
                &format!("connection_id_{app_id}_{i}_2").to_string(),
                &app_id,
                &network.to_string(),
            )
            .await
            .unwrap();

            for j in 0..i {
                db.create_new_connection_by_client(
                    &mut tx,
                    &app_id,
                    &session_id,
                    j as i64,
                    &network.to_string(),
                )
                .await
                .unwrap();
            }

            tx.commit().await.unwrap();
        }

        // Manually refresh the continuous aggregates
        db.refresh_continuous_aggregates(vec![
            "hourly_connection_stats_per_app_and_network".to_string(),
            "daily_connection_stats_per_app_and_network".to_string(),
            "monthly_connection_stats_per_app_and_network".to_string(),
        ])
        .await
        .unwrap();

        // Get stats for all networks
        let stats = db
            .get_connections_stats_by_app_id(&app_id, None, TimeFilter::LastMonth)
            .await
            .unwrap();

        for (i, network) in networks.iter().enumerate() {
            let network_stats = stats.iter().find(|s| s.network == *network).unwrap();
            assert_eq!(network_stats.app_connection_count, 3);
            assert_eq!(network_stats.clients_connection_count, i as i64);
        }
    }
}
