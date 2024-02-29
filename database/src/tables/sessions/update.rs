use super::table_struct::{DbNcSession, SESSIONS_KEYS, SESSIONS_TABLE_NAME};
use crate::{db::Db, structs::client_data::ClientData};
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
    ) -> Result<(), sqlx::Error> {
        let mut tx = self.connection_pool.begin().await.unwrap();

        // 1. Save the new session
        if let Err(err) = self.save_new_session(&mut tx, &session).await {
            tx.rollback().await.unwrap();
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
            tx.rollback().await.unwrap();
            return Err(err);
        }

        tx.commit().await.unwrap();

        Ok(())
    }

    pub async fn save_new_session(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        session: &DbNcSession,
    ) -> Result<(), sqlx::Error> {
        let query_body = format!(
            "INSERT INTO {SESSIONS_TABLE_NAME} ({}) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)",
            SESSIONS_KEYS
        );

        let (client_id, device, metadata, notification_endpoint, connected_at) =
            match &session.client {
                Some(client) => (
                    &client.client_id,
                    &client.device,
                    &client.metadata,
                    &client.notification_endpoint,
                    Some(client.connected_at.clone()),
                ),
                None => (&None, &None, &None, &None, None),
            };

        let query_result = query(&query_body)
            .bind(&session.session_id)
            .bind(&session.app_id)
            .bind(&session.app_metadata)
            .bind(&session.app_ip_address)
            .bind(&session.persistent)
            .bind(&session.network)
            .bind(&client_id)
            .bind(&device)
            .bind(&metadata)
            .bind(&notification_endpoint)
            .bind(&connected_at)
            .bind(&session.session_open_timestamp)
            .bind(&None::<DateTime<Utc>>)
            .execute(&mut **tx)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn close_session(
        &self,
        session_id: &String,
        close_timestamp: DateTime<Utc>,
    ) -> Result<(), sqlx::Error> {
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
            Err(e) => Err(e),
        }
    }

    pub async fn connect_user_to_the_session(
        &self,
        client_data: &ClientData,
        connected_keys: &Vec<String>,
        session_id: &String,
    ) -> Result<(), sqlx::Error> {
        // Start a new transaction
        let mut tx = self.connection_pool.begin().await.unwrap();

        // User can't connect to the session without any connected keys
        if connected_keys.is_empty() {
            // TODO for now return this error, replace with a custom error
            return Err(sqlx::Error::RowNotFound);
        }

        // 1. Handle connected keys
        let mut client_profile_id: Option<i64> = None;
        for key in connected_keys {
            match self.handle_public_key_entry(&mut tx, &key).await {
                Ok(profile_id) => {
                    client_profile_id = Some(profile_id);
                }
                Err(err) => {
                    tx.rollback().await.unwrap();
                    return Err(err);
                }
            }
        }

        // It is not possible to not get client profile id at this point
        if client_profile_id.is_none() {
            // TODO for now return this error, replace with a custom error
            return Err(sqlx::Error::RowNotFound);
        }

        // 2. Update the session with the client data
        let query_body = format!(
            "UPDATE {SESSIONS_TABLE_NAME} SET client_id = $1, client_device = $2, client_metadata = $3, client_notification_endpoint = $4, client_connected_at = $5, client_profile_id = $6 WHERE session_id = $7"
        );

        let query_result = query(&query_body)
            .bind(&client_data.client_id)
            .bind(&client_data.device)
            .bind(&client_data.metadata)
            .bind(&client_data.notification_endpoint)
            .bind(&client_data.connected_at)
            .bind(&client_profile_id)
            .bind(&session_id)
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

    use super::*;
    use crate::{
        structs::{client_data::ClientData, request_status::RequestStatus},
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
            app_id: "test_app_id".to_string(),
            app_metadata: "test_app_metadata".to_string(),
            app_ip_address: "test_app_ip_address".to_string(),
            persistent: false,
            network: "test_network".to_string(),
            client_profile_id: None,
            client: Some(ClientData {
                client_id: Some("test_client_id".to_string()),
                device: Some("test_device".to_string()),
                metadata: Some("test_metadata".to_string()),
                notification_endpoint: Some("test_notification_endpoint".to_string()),
                connected_at: get_date_time(10).unwrap(),
            }),
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
