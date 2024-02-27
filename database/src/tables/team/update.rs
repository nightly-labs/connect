use super::table_struct::TEAM_KEYS;
use crate::{
    db::Db,
    structs::subscription::Subscription,
    tables::{
        registered_app::table_struct::RegisteredApp,
        team::table_struct::{Team, TEAM_TABLE_NAME},
    },
};
use sqlx::{query, Transaction};

impl Db {
    pub async fn create_new_team_within_tx(
        &self,
        tx: &mut Transaction<'_, sqlx::Postgres>,
        team: &Team,
    ) -> Result<(), sqlx::Error> {
        let query_body =
            format!("INSERT INTO {TEAM_TABLE_NAME} ({TEAM_KEYS}) VALUES ($1, $2, $3, $4)");

        let query_result = query(&query_body)
            .bind(&team.team_id)
            .bind(&team.subscription)
            .bind(&team.team_admin_id)
            .bind(&team.creation_timestamp)
            .execute(&mut **tx)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn update_subscription(
        &self,
        team_id: &String,
        subscription: &Subscription,
    ) -> Result<(), sqlx::Error> {
        let query_body =
            format!("UPDATE {TEAM_TABLE_NAME} SET subscription = $1 WHERE team_id = $2");
        let query_result = query(&query_body)
            .bind(subscription)
            .bind(team_id)
            .execute(&self.connection_pool)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn create_team_and_register_app(
        &self,
        team: &Team,
        app: &RegisteredApp,
    ) -> Result<(), sqlx::Error> {
        // Start a transaction
        let mut tx: Transaction<'_, sqlx::Postgres> = self.connection_pool.begin().await?;

        // Attempt to create the new team within the transaction
        let create_team_result = self.create_new_team_within_tx(&mut tx, team).await;
        if create_team_result.is_err() {
            // If creating the team fails, roll back the transaction and return the error
            tx.rollback().await?;
            return create_team_result;
        }

        // Attempt to register the new app within the same transaction
        let register_app_result = self.register_new_app_within_tx(&mut tx, app).await;
        if register_app_result.is_err() {
            // If registering the app fails, roll back the transaction and return the error
            tx.rollback().await?;
            return register_app_result;
        }

        // If both actions succeeded, commit the transaction
        tx.commit().await?;
        Ok(())
    }
}

// #[cfg(test)]
// mod tests {
//     use crate::{
//         structs::{
//             consts::DAY_IN_SECONDS, request_status::RequestStatus, time_filters::TimeFilter,
//         },
//         tables::{
//             registered_app::table_struct::RegisteredApp, requests::table_struct::Request,
//             sessions::table_struct::DbNcSession,
//         },
//     };
//     use sqlx::types::chrono::{DateTime, Utc};
//     use std::{sync::Arc, time::Duration};
//     use tokio::task;

//     #[tokio::test]
//     async fn test_create_team() {
//         let db = super::Db::connect_to_the_pool().await;
//         db.truncate_all_tables().await.unwrap();

//         // "Register" an app
//         let app = RegisteredApp {
//             app_id: "test_app_id".to_string(),
//             app_name: "test_app_name".to_string(),
//             whitelisted_domains: vec!["test_domain".to_string()],
//             subscription: None,
//             ack_public_keys: vec!["test_key".to_string()],
//             email: None,
//             registration_timestamp: 0,
//             pass_hash: None,
//         };

//         db.register_new_app(&app).await.unwrap();

//         let result = db.get_registered_app_by_app_id(&app.app_id).await.unwrap();
//         assert_eq!(app, result);

//         // Create session
//         let session = DbNcSession {
//             session_id: "test_session_id".to_string(),
//             app_id: "test_app_id".to_string(),
//             app_metadata: "test_app_metadata".to_string(),
//             app_ip_address: "test_app_ip_address".to_string(),
//             persistent: false,
//             network: "test_network".to_string(),
//             client: None,
//             session_open_timestamp: DateTime::from(Utc::now()),
//             session_close_timestamp: None,
//         };

//         db.save_new_session(&session).await.unwrap();

//         let result = db.get_sessions_by_app_id(&app.app_id).await.unwrap();
//         assert_eq!(result.len(), 1);
//         // assert_eq!(session, result[0]);

//         let db_arc = Arc::new(db);
//         let mut tasks = Vec::new();

//         for i in 0..33 {
//             let db_clone = db_arc.clone(); // Clone the db connection or pool if needed
//             tasks.push(task::spawn(async move {
//                 for j in 0..100 - i {
//                     let creation_time: DateTime<Utc> = Utc::now()
//                         - Duration::from_secs(i as u64 * DAY_IN_SECONDS as u64)
//                         - Duration::from_millis((j + 1) as u64 * 100);

//                     let request = Request {
//                         request_id: format!("test_request_id_{}_{}", i, j),
//                         app_id: "test_app_id".to_string(),
//                         session_id: "test_session_id".to_string(),
//                         network: "test_network".to_string(),
//                         creation_timestamp: creation_time,
//                         request_status: RequestStatus::Pending,
//                         request_type: "test_request_type".to_string(),
//                     };

//                     if let Err(e) = db_clone.save_request(&request).await {
//                         eprintln!("Failed to save request: {}", e);
//                     }
//                 }
//             }));
//         }

//         // Await all tasks to complete
//         for task in tasks {
//             task.await.unwrap();
//         }

//         // We need to refresh manually the views
//         db_arc
//             .refresh_continuous_aggregates(vec![
//                 "hourly_requests_stats".to_string(),
//                 "daily_requests_stats".to_string(),
//                 "monthly_requests_stats".to_string(),
//             ])
//             .await
//             .unwrap();

//         let result = db_arc
//             .get_requests_stats_by_app_id(&app.app_id, TimeFilter::Last24Hours)
//             .await
//             .unwrap();

//         assert_eq!(result.len(), 2);
//         assert_eq!(result[0].request_count, 100);
//         assert_eq!(result[1].request_count, 99);

//         let result = db_arc
//             .get_requests_stats_by_app_id(&app.app_id, TimeFilter::Last7Days)
//             .await
//             .unwrap();

//         assert_eq!(result.len(), 8);
//         assert_eq!(result[0].request_count, 100);
//         assert_eq!(result[7].request_count, 93);

//         let result = db_arc
//             .get_requests_stats_by_app_id(&app.app_id, TimeFilter::Last30Days)
//             .await
//             .unwrap();

//         assert_eq!(result.len(), 31);
//         assert_eq!(result[0].request_count, 100);
//         assert_eq!(result[30].request_count, 70);
//     }
// }
