/////////////////////////////////////// For now this will be commented out, as it might be used in nearby future.

// use crate::{db::Db, structs::filter_requests::SessionAvgTime};
// use sqlx::Error;

// pub const MONTHLY_SESSIONS_AVG_TIME_PER_APP_VIEW_NAME: &str = "monthly_sessions_avg_time_per_app";
// pub const MONTHLY_SESSIONS_AVG_TIME_PER_APP_KEYS: &str =
//     "app_id, monthly_bucket as bucket, monthly_avg_session_duration_seconds as average_duration_seconds";

// impl Db {
//     pub async fn get_monthly_avg_session_duration_per_app(
//         &self,
//         app_id: &str,
//     ) -> Result<Vec<SessionAvgTime>, Error> {
//         let query = format!(
//             "SELECT {MONTHLY_SESSIONS_AVG_TIME_PER_APP_KEYS}
//             FROM {MONTHLY_SESSIONS_AVG_TIME_PER_APP_VIEW_NAME}
//             WHERE app_id = $1"
//         );

//         sqlx::query_as::<_, SessionAvgTime>(&query)
//             .bind(app_id)
//             .fetch_all(&self.connection_pool)
//             .await
//     }
// }

// #[cfg(feature = "cloud_integration_tests")]
// #[cfg(test)]
// mod test {

//     use crate::tables::sessions::table_struct::DbNcSession;
//     use sqlx::types::chrono::Utc;
//     use std::time::Duration;

//     #[tokio::test]
//     async fn test_average_session_duration() {
//         let db = super::Db::connect_to_the_pool().await;
//         db.truncate_all_tables().await.unwrap();

//         // Create test team instance
//         let team_id = "test_team_id".to_string();
//         let app_id = "test_app_id".to_string();

//         db.setup_test_team(&team_id, &app_id, Utc::now())
//             .await
//             .unwrap();

//         // Create sessions
//         let now = Utc::now();
//         let sessions = vec![
//             (
//                 now - Duration::from_secs(4 * 60 * 60),
//                 now - Duration::from_secs((4 * 60 - 85) * 60),
//             ), // 1 hour 25 minutes session, 4 hours ago
//             (
//                 now - Duration::from_secs((2 * 60 + 35) * 60),
//                 now - Duration::from_secs((2 * 60 + 10) * 60),
//             ), // 25 minutes session, 2 hours and 35 minutes ago
//             (
//                 now - Duration::from_secs((1 * 60 + 50) * 60),
//                 now - Duration::from_secs((1 * 60 + 40) * 60),
//             ), // 10 minutes session, 1 hour and 50 minutes ago
//         ];

//         for (start, end) in sessions.iter() {
//             let session = DbNcSession {
//                 session_id: format!("session_id_{}", start.timestamp()),
//                 app_id: "test_app_id".to_string(),
//                 app_metadata: "test_app_metadata".to_string(),
//                 persistent: false,
//                 network: "test_network".to_string(),
//                 client_data: None,
//                 session_open_timestamp: start.clone(),
//                 session_close_timestamp: None,
//             };

//             db.handle_new_session(&session, None, &"127.0.0.1".to_string(), &start)
//                 .await
//                 .unwrap();
//             db.close_session(&session.session_id, *end).await.unwrap();
//         }

//         // We need to refresh manually the views
//         db.refresh_continuous_aggregates(vec![
//             "daily_sessions_avg_time_per_app".to_string(),
//             "monthly_sessions_avg_time_per_app".to_string(),
//         ])
//         .await
//         .unwrap();

//         let result = db
//             .get_monthly_avg_session_duration_per_app(&app_id)
//             .await
//             .unwrap();

//         assert_eq!(result.len(), 1);

//         let expected_avg_duration_seconds: f64 = sessions
//             .iter()
//             .map(|(start, end)| (end.timestamp() - start.timestamp()) as f64)
//             .sum::<f64>()
//             / sessions.len() as f64;

//         assert_eq!(
//             result[0].average_duration_seconds,
//             expected_avg_duration_seconds
//         );
//     }
// }
