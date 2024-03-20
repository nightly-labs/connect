use crate::db::Db;
use crate::structs::db_error::DbError;
use crate::structs::entity_type::EntityType;
use crate::structs::geo_location::Geolocation;
use crate::structs::session_type::SessionType;
use crate::tables::connection_events::table_struct::{
    CONNECTION_EVENTS_KEYS_KEYS, CONNECTION_EVENTS_TABLE_NAME,
};
use sqlx::types::chrono::{DateTime, Utc};
use sqlx::Transaction;
use sqlx::{query, Postgres};

impl Db {
    pub async fn create_new_connection_event_by_app(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        session_id: &String,
        app_id: &String,
        ip: &String,
        geo_location: Option<Geolocation>,
    ) -> Result<(), DbError> {
        let query_body = format!(
            "INSERT INTO {CONNECTION_EVENTS_TABLE_NAME} ({CONNECTION_EVENTS_KEYS_KEYS}) VALUES (DEFAULT, $1, $2, $3, $4, $5, $6, $7, $8, NOW(), NULL)"
        );

        let query_result = query(&query_body)
            .bind(&app_id)
            .bind(&session_id)
            .bind(&app_id)
            .bind(&EntityType::App)
            .bind(&ip)
            .bind(None::<SessionType>)
            .bind(geo_location)
            // initialize connect event with true success flag
            .bind(true)
            .execute(&mut **tx)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e).map_err(|e| e.into()),
        }
    }

    pub async fn create_new_connection_event_by_client(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        app_id: &String,
        session_id: &String,
        session_type: &SessionType,
        ip: &String,
        geo_location: Option<Geolocation>,
    ) -> Result<(), DbError> {
        let query_body = format!(
            "INSERT INTO {CONNECTION_EVENTS_TABLE_NAME} ({CONNECTION_EVENTS_KEYS_KEYS}) VALUES (DEFAULT, $1, $2, $3, $4, $5, $6, $7, $8, NOW(), NULL)"
        );

        // If user gets connected, the "client_id" will get replaced with proper client_profile_id derived from user's public key
        let client_id = "Temporary id".to_string();

        let query_result = query(&query_body)
            .bind(&app_id)
            .bind(&session_id)
            .bind(&client_id)
            .bind(&EntityType::Client)
            .bind(&ip)
            .bind(Some(session_type))
            .bind(geo_location)
            // initialize connect event with false success flag, only update to true if we receive a successful connection event
            .bind(false)
            .execute(&mut **tx)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e).map_err(|e| e.into()),
        }
    }

    pub async fn resolve_successful_connection_by_client(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        app_id: &String,
        session_id: &String,
        client_profile_id: i64,
        connected_at: &DateTime<Utc>,
    ) -> Result<(), DbError> {
        // First thing, get most recent connection event by client for combination of app_id and session_id with success flag set to false
        let last_connection_event = match self
            .get_last_connection_attempt_by_client(app_id, session_id)
            .await
        {
            Ok(connection_event) => match connection_event {
                Some(connection_event) => connection_event,
                None => {
                    return Err(DbError::DatabaseError(format!(
                        "No connection event found for the given app_id: [{}] and session_id: [{}]",
                        app_id, session_id
                    )));
                }
            },
            Err(e) => {
                return Err(e);
            }
        };

        // Update the fields of the last connection event
        let query_body = format!(
            "UPDATE {CONNECTION_EVENTS_TABLE_NAME} SET success = true, entity_id = $1, connected_at = $2 WHERE event_id = $3"
        );

        let query_result = query(&query_body)
            .bind(&client_profile_id)
            .bind(&connected_at)
            .bind(&last_connection_event.event_id)
            .execute(&mut **tx)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e).map_err(|e| e.into()),
        }
    }

    pub async fn close_app_connection(
        &self,
        session_id: &String,
        app_id: &String,
    ) -> Result<(), DbError> {
        // If there is more than 1 concurrent connection, we will close all of them
        let query_body = format!(
            "UPDATE {CONNECTION_EVENTS_TABLE_NAME} 
                SET disconnected_at = NOW() 
                WHERE app_id = $1 AND session_id = $2 AND entity_type = $3 AND entity_id = $4 AND disconnected_at IS NULL"
        );

        let query_result = query(&query_body)
            .bind(&app_id)
            .bind(&session_id)
            .bind(&EntityType::App)
            .bind(&app_id)
            .execute(&self.connection_pool)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e).map_err(|e| e.into()),
        }
    }

    pub async fn close_client_connection(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        app_id: &String,
        session_id: &String,
    ) -> Result<(), DbError> {
        let query_body = format!(
            "UPDATE {CONNECTION_EVENTS_TABLE_NAME} 
                SET disconnected_at = NOW() 
                WHERE app_id = $1 AND session_id = $2 AND entity_type = $3 AND success = true AND disconnected_at IS NULL"
        );

        let query_result = query(&query_body)
            .bind(&app_id)
            .bind(&session_id)
            .bind(&EntityType::Client)
            .execute(&mut **tx)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e).map_err(|e| e.into()),
        }
    }
}

// TODO fix those tests

// #[cfg(test)]
// mod tests {

//     use super::*;
//     use crate::{
//         structs::client_data::ClientData,
//         tables::{sessions::table_struct::DbNcSession, utils::to_microsecond_precision},
//     };
//     use sqlx::types::chrono::{DateTime, Utc};
//     use std::collections::HashMap;

//     #[tokio::test]
//     async fn test_connection_events() {
//         let db = Db::connect_to_the_pool().await;
//         db.truncate_all_tables().await.unwrap();

//         let mut tx = db.connection_pool.begin().await.unwrap();

//         let session_id = "session_id".to_string();
//         let app_id = "app_id".to_string();
//         let network = "network".to_string();

//         // Create event by app
//         db.create_new_connection_event_by_app(&mut tx, &session_id, &app_id, &network, None)
//             .await
//             .unwrap();

//         // Create event by client
//         let client_profile_id = 1;
//         db.create_new_connection_event_by_client(
//             &mut tx,
//             &app_id,
//             &session_id,
//             &SessionType::Relay,
//             &network,
//             None,
//         )
//         .await
//         .unwrap();

//         tx.commit().await.unwrap();

//         // Get events by session id
//         let events_by_session_id = db
//             .get_connection_events_by_session_id(&session_id)
//             .await
//             .unwrap();

//         assert_eq!(events_by_session_id.len(), 2);

//         // Get events by app id
//         let events_by_app_id = db.get_connection_events_by_app(&app_id).await.unwrap();

//         assert_eq!(events_by_app_id.len(), 1);

//         // Get events by client profile id
//         let events_by_client_profile_id = db
//             .get_connection_events_by_client_profile_id(&client_profile_id.to_string())
//             .await
//             .unwrap();

//         assert_eq!(events_by_client_profile_id.len(), 1);

//         // Add another connection event by app
//         let mut tx = db.connection_pool.begin().await.unwrap();
//         db.create_new_connection_event_by_app(&mut tx, &session_id, &app_id, &network, None)
//             .await
//             .unwrap();

//         tx.commit().await.unwrap();

//         // Get events by app id
//         let events_by_app_id = db.get_connection_events_by_app_id(&app_id).await.unwrap();

//         assert_eq!(events_by_app_id.len(), 3);

//         events_by_app_id.iter().for_each(|event| {
//             assert!(event.disconnected_at.is_none());
//         });

//         // Close app first connection
//         db.close_app_connection(&session_id, &app_id).await.unwrap();

//         // Get events by app id
//         let events_by_app_id = db.get_connection_events_by_app_id(&app_id).await.unwrap();

//         assert_eq!(events_by_app_id.len(), 3);

//         events_by_app_id.iter().for_each(|event| {
//             if event.entity_type == EntityType::App {
//                 assert!(event.disconnected_at.is_some());
//             } else {
//                 assert!(event.disconnected_at.is_none());
//             }
//         });

//         // Close remaining connections
//         let mut tx = db.connection_pool.begin().await.unwrap();
//         db.close_client_connection(&mut tx, &app_id, &session_id, client_profile_id)
//             .await
//             .unwrap();
//         tx.commit().await.unwrap();

//         // Get events by app id
//         let events_by_app_id = db.get_connection_events_by_app_id(&app_id).await.unwrap();

//         assert_eq!(events_by_app_id.len(), 3);
//         events_by_app_id.iter().for_each(|event| {
//             assert!(event.disconnected_at.is_some());
//         });
//     }

//     #[tokio::test]
//     async fn get_all_users() {
//         let db = Db::connect_to_the_pool().await;
//         db.truncate_all_tables().await.unwrap();

//         // Create test team instance
//         let team_id = "test_team_id".to_string();
//         let app_id = "test_app_id".to_string();

//         db.setup_test_team(&team_id, &app_id, Utc::now())
//             .await
//             .unwrap();

//         let session = DbNcSession {
//             session_id: "test_session_id".to_string(),
//             app_id: "test_app_id".to_string(),
//             app_metadata: "test_app_metadata".to_string(),
//             persistent: false,
//             network: "test_network".to_string(),
//             client_data: None,
//             session_open_timestamp: to_microsecond_precision(&Utc::now()),
//             session_close_timestamp: None,
//         };

//         // Create a new session entry
//         db.handle_new_session(&session, None, &"127.0.0.1".to_string())
//             .await
//             .unwrap();

//         let first_client_data = ClientData {
//             client_id: "first_client_id".to_string(),
//             connected_at: to_microsecond_precision(&Utc::now()),
//             client_profile_id: 1,
//             wallet_name: "test_wallet_name".to_string(),
//             wallet_type: "test_wallet_type".to_string(),
//         };
//         let first_user_keys = vec![
//             "first_key".to_string(),
//             "second_key".to_string(),
//             "third_key".to_string(),
//         ];

//         db.connect_user_to_the_session(
//             &first_client_data.client_id,
//             &first_client_data.wallet_name,
//             &first_client_data.wallet_type,
//             &first_client_data.connected_at,
//             &first_user_keys,
//             &app_id,
//             &session.session_id,
//         )
//         .await
//         .unwrap();

//         let second_client_data = ClientData {
//             client_id: "second_client_id".to_string(),
//             connected_at: to_microsecond_precision(&Utc::now()),
//             client_profile_id: 2,
//             wallet_name: "test_wallet_name".to_string(),
//             wallet_type: "test_wallet_type".to_string(),
//         };
//         let second_user_keys = vec!["fourth_key".to_string(), "sixth_key".to_string()];

//         db.connect_user_to_the_session(
//             &second_client_data.client_id,
//             &second_client_data.wallet_name,
//             &second_client_data.wallet_type,
//             &second_client_data.connected_at,
//             &second_user_keys,
//             &app_id,
//             &session.session_id,
//         )
//         .await
//         .unwrap();

//         let third_client_data = ClientData {
//             client_id: "third_client_id".to_string(),
//             connected_at: to_microsecond_precision(&Utc::now()),
//             client_profile_id: 3,
//             wallet_name: "test_wallet_name".to_string(),
//             wallet_type: "test_wallet_type".to_string(),
//         };
//         let third_user_keys = vec!["seventh_key".to_string()];
//         db.connect_user_to_the_session(
//             &third_client_data.client_id,
//             &third_client_data.wallet_name,
//             &third_client_data.wallet_type,
//             &third_client_data.connected_at,
//             &third_user_keys,
//             &app_id,
//             &session.session_id,
//         )
//         .await
//         .unwrap();

//         // Get all connected users
//         let connected_users = db.get_all_app_distinct_users(&app_id).await.unwrap();

//         assert_eq!(connected_users.len(), 3);
//         let first_connection_hashmap = connected_users
//             .iter()
//             .map(|user| {
//                 (
//                     user.public_key.clone(),
//                     (user.first_connection, user.last_connection),
//                 )
//             })
//             .collect::<HashMap<String, (DateTime<Utc>, DateTime<Utc>)>>();

//         // Connect as first user again
//         db.connect_user_to_the_session(
//             &first_client_data.client_id,
//             &first_client_data.wallet_name,
//             &first_client_data.wallet_type,
//             &first_client_data.connected_at,
//             &first_user_keys,
//             &app_id,
//             &session.session_id,
//         )
//         .await
//         .unwrap();

//         // Connect as second user again
//         db.connect_user_to_the_session(
//             &second_client_data.client_id,
//             &second_client_data.wallet_name,
//             &second_client_data.wallet_type,
//             &second_client_data.connected_at,
//             &second_user_keys,
//             &app_id,
//             &session.session_id,
//         )
//         .await
//         .unwrap();

//         // Get all connected users
//         let connected_users = db.get_all_app_distinct_users(&app_id).await.unwrap();

//         assert_eq!(connected_users.len(), 3);
//         let second_connection_hashmap = connected_users
//             .iter()
//             .map(|user| {
//                 (
//                     user.public_key.clone(),
//                     (user.first_connection, user.last_connection),
//                 )
//             })
//             .collect::<HashMap<String, (DateTime<Utc>, DateTime<Utc>)>>();

//         // Connect as third user again
//         db.connect_user_to_the_session(
//             &third_client_data.client_id,
//             &third_client_data.wallet_name,
//             &third_client_data.wallet_type,
//             &third_client_data.connected_at,
//             &third_user_keys,
//             &app_id,
//             &session.session_id,
//         )
//         .await
//         .unwrap();

//         // Get all connected users
//         let connected_users = db.get_all_app_distinct_users(&app_id).await.unwrap();

//         assert_eq!(connected_users.len(), 3);
//         let third_connection_hashmap = connected_users
//             .iter()
//             .map(|user| {
//                 (
//                     user.public_key.clone(),
//                     (user.first_connection, user.last_connection),
//                 )
//             })
//             .collect::<HashMap<String, (DateTime<Utc>, DateTime<Utc>)>>();

//         // Check users, each of them should have been identified by the first key the provided

//         // Check first user,
//         // First connection
//         assert!(
//             first_connection_hashmap.get("first_key").unwrap().0
//                 <= second_connection_hashmap.get("first_key").unwrap().0
//         );
//         // Last connection
//         assert!(
//             second_connection_hashmap.get("first_key").unwrap().1
//                 >= first_connection_hashmap.get("first_key").unwrap().1
//         );
//         // Last connection should have stayed the same
//         assert!(
//             second_connection_hashmap.get("first_key").unwrap().1
//                 == third_connection_hashmap.get("first_key").unwrap().1
//         );

//         // Check second user,
//         // First connection
//         assert!(
//             first_connection_hashmap.get("fourth_key").unwrap().0
//                 <= second_connection_hashmap.get("fourth_key").unwrap().0
//         );
//         // Last connection
//         assert!(
//             second_connection_hashmap.get("fourth_key").unwrap().1
//                 >= first_connection_hashmap.get("fourth_key").unwrap().1
//         );
//         // Last connection should have stayed the same
//         assert!(
//             second_connection_hashmap.get("fourth_key").unwrap().1
//                 == third_connection_hashmap.get("fourth_key").unwrap().1
//         );

//         // Check third user,
//         // First connection
//         assert!(
//             first_connection_hashmap.get("seventh_key").unwrap().0
//                 == third_connection_hashmap.get("seventh_key").unwrap().0
//         );
//         // Last connection
//         assert!(
//             third_connection_hashmap.get("seventh_key").unwrap().1
//                 >= second_connection_hashmap.get("seventh_key").unwrap().1
//         );
//     }
// }
