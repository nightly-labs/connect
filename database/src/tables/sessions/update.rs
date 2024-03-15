use super::table_struct::{DbNcSession, SESSIONS_KEYS, SESSIONS_TABLE_NAME};
use crate::{
    db::Db,
    structs::{client_data::ClientData, db_error::DbError},
};
use log::error;
use sqlx::{
    query,
    types::chrono::{DateTime, Utc},
    Postgres, Transaction,
};

impl Db {
    pub async fn handle_new_session(
        &self,
        session: &DbNcSession,
        connection_id: &String,
    ) -> Result<(), DbError> {
        let mut tx = self.connection_pool.begin().await.unwrap();

        // 1. Save the new session
        if let Err(err) = self.save_new_session(&mut tx, &session).await {
            let _ = tx
                .rollback()
                .await
                .map_err(|err| error!("Failed to rollback transaction: {:?}", err));
            return Err(err);
        }

        // 2. Add new app connection event
        if let Err(err) = self
            .create_new_connection_event_by_app(
                &mut tx,
                &session.session_id,
                &connection_id,
                &session.app_id,
                &session.network,
            )
            .await
        {
            let _ = tx
                .rollback()
                .await
                .map_err(|err| error!("Failed to rollback transaction: {:?}", err));
            return Err(err);
        }

        tx.commit().await.unwrap();

        Ok(())
    }

    pub async fn save_new_session(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        session: &DbNcSession,
    ) -> Result<(), DbError> {
        let query_body = format!(
            "INSERT INTO {SESSIONS_TABLE_NAME} ({}) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)",
            SESSIONS_KEYS
        );

        let query_result = query(&query_body)
            .bind(&session.session_id)
            .bind(&session.session_type)
            .bind(&session.app_id)
            .bind(&session.app_metadata)
            .bind(&session.persistent)
            .bind(&session.network)
            .bind(&None::<i64>)
            .bind(&None::<String>)
            .bind(&None::<String>)
            .bind(&None::<String>)
            .bind(&None::<String>)
            .bind(&None::<DateTime<Utc>>)
            .bind(&session.session_open_timestamp)
            .bind(&None::<DateTime<Utc>>)
            .execute(&mut **tx)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e).map_err(|e| e.into()),
        }
    }

    pub async fn close_session(
        &self,
        session_id: &String,
        close_timestamp: DateTime<Utc>,
    ) -> Result<(), DbError> {
        let query_body = format!(
            "UPDATE {SESSIONS_TABLE_NAME} SET session_close_timestamp = $1 WHERE session_id = $2"
        );

        let query_result = query(&query_body)
            .bind(close_timestamp)
            .bind(session_id)
            .execute(&self.connection_pool)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e).map_err(|e| e.into()),
        }
    }

    pub async fn connect_user_to_the_session(
        &self,
        client_data: &ClientData,
        connected_keys: &Vec<String>,
        app_id: &String,
        session_id: &String,
        network: &String,
    ) -> Result<(), DbError> {
        // Start a new transaction
        let mut tx = self.connection_pool.begin().await.unwrap();

        // User can't connect to the session without any connected keys
        if connected_keys.is_empty() {
            return Err(DbError::DatabaseError(
                "No connected keys provided".to_string(),
            ));
        }

        // 1. Handle connected keys
        let (client_profile_id, used_public_key) = match self
            .handle_public_keys_entries(&mut tx, &connected_keys)
            .await
        {
            Ok(profile_id) => profile_id,
            Err(err) => {
                let _ = tx
                    .rollback()
                    .await
                    .map_err(|err| error!("Failed to rollback transaction: {:?}", err));
                return Err(err);
            }
        };

        // 2. Update the session with the client data
        let query_body =
            format!("UPDATE {SESSIONS_TABLE_NAME} SET client_data = $1, WHERE session_id = $2");

        let query_result = query(&query_body)
            .bind(&client_data)
            .bind(&session_id)
            .execute(&self.connection_pool)
            .await;

        if let Err(err) = query_result {
            let _ = tx
                .rollback()
                .await
                .map_err(|err| error!("Failed to rollback transaction: {:?}", err));
            return Err(err).map_err(|e| e.into());
        }

        // 3. Create new session public key entry for each connected key
        for key in connected_keys {
            if key == &used_public_key {
                // For the used key, create a new session public key entry which will be marked as the main session key
                if let Err(err) = self
                    .create_new_session_public_key(
                        &mut tx,
                        &session_id,
                        key.clone(),
                        Some(client_profile_id),
                        true,
                    )
                    .await
                {
                    let _ = tx
                        .rollback()
                        .await
                        .map_err(|err| error!("Failed to rollback transaction: {:?}", err));
                    return Err(err);
                }
            } else {
                // For the rest of the keys, create a new session public key entry which would act as a soft relation between used keys and the profiles to which they belong
                // Check if the key is already used by the client, this might be dangerous in case somebody spams the connection with lots of keys
                let client_profile_id_option = self
                    .get_public_key(&key)
                    .await
                    .ok()
                    .map(|key| key.client_profile_id);

                if let Err(err) = self
                    .create_new_session_public_key(
                        &mut tx,
                        &session_id,
                        key.clone(),
                        client_profile_id_option,
                        false,
                    )
                    .await
                {
                    let _ = tx
                        .rollback()
                        .await
                        .map_err(|err| error!("Failed to rollback transaction: {:?}", err));
                    return Err(err);
                }
            }
        }

        // 4. Create new connection event
        if let Err(err) = self
            .create_new_connection_by_client(
                &mut tx,
                &app_id,
                &session_id,
                client_profile_id,
                &network,
            )
            .await
        {
            let _ = tx
                .rollback()
                .await
                .map_err(|err| error!("Failed to rollback transaction: {:?}", err));
            return Err(err);
        }

        tx.commit().await.unwrap();

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{
        structs::{request_status::RequestStatus, session_type::SessionType},
        tables::{requests::table_struct::Request, utils::get_date_time},
    };

    #[tokio::test]
    async fn test_sessions() {
        let db = super::Db::connect_to_the_pool().await;
        db.truncate_all_tables().await.unwrap();

        // Create test team instance
        let team_id = "test_team_id".to_string();
        let app_id = "test_app_id".to_string();

        db.setup_test_team(&team_id, &app_id, Utc::now())
            .await
            .unwrap();

        let session = DbNcSession {
            session_id: "test_session_id".to_string(),
            session_type: SessionType::Relay,
            app_id: "test_app_id".to_string(),
            app_metadata: "test_app_metadata".to_string(),
            persistent: false,
            network: "test_network".to_string(),
            client_data: None,
            session_open_timestamp: get_date_time(10).unwrap(),
            session_close_timestamp: None,
        };

        // Create a new session entry
        db.handle_new_session(&session, &"connection_id".to_string())
            .await
            .unwrap();

        // Get all sessions by app_id
        let sessions = db.get_sessions_by_app_id(&session.app_id).await.unwrap();
        assert_eq!(sessions.len(), 1);
        assert_eq!(session, sessions[0]);

        // Get session by session_id
        let session = db
            .get_session_by_session_id(&session.session_id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(session, session);

        // Change the session status to closed
        db.close_session(&session.session_id, get_date_time(15).unwrap())
            .await
            .unwrap();

        // Get session by session_id to check if the session status is closed
        let session = db
            .get_session_by_session_id(&session.session_id)
            .await
            .unwrap()
            .unwrap();
        assert_eq!(session.session_close_timestamp, get_date_time(15));

        // Create a few requests for the session
        let request = Request {
            request_id: "test_request_id".to_string(),
            request_type: "test_request_type".to_string(),
            app_id: "test_app_id".to_string(),
            session_id: "test_session_id".to_string(),
            request_status: RequestStatus::Pending,
            network: "test_network".to_string(),
            creation_timestamp: get_date_time(12).unwrap(),
        };

        let second_request = Request {
            request_id: "test_request_id2".to_string(),
            request_type: "test_request_type".to_string(),
            session_id: "test_session_id".to_string(),
            app_id: "test_app_id".to_string(),
            request_status: RequestStatus::Pending,
            network: "test_network".to_string(),
            creation_timestamp: get_date_time(13).unwrap(),
        };

        db.save_request(&request).await.unwrap();
        db.save_request(&second_request).await.unwrap();

        // Get all requests by session_id
        let requests = db
            .get_requests_by_session_id(&request.session_id)
            .await
            .unwrap();

        assert_eq!(requests.len(), 2);
        assert_eq!(request, requests[1]);
        assert_eq!(second_request, requests[0]);
    }
}
