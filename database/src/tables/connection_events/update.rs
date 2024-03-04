use crate::db::Db;
use crate::structs::entity_type::EntityType;
use crate::tables::connection_events::table_struct::{
    CONNECTION_EVENTS_KEYS_KEYS, CONNECTION_EVENTS_TABLE_NAME,
};
use sqlx::Transaction;
use sqlx::{query, Postgres};

impl Db {
    pub async fn create_new_connection_event_by_app(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        session_id: &String,
        connection_id: &String,
        app_id: &String,
        network: &String,
    ) -> Result<(), sqlx::Error> {
        let query_body = format!(
            "INSERT INTO {CONNECTION_EVENTS_TABLE_NAME} ({CONNECTION_EVENTS_KEYS_KEYS}) VALUES (DEFAULT, $1, $2, $3, $4, $5, $6, NOW(), NULL)"
        );

        let query_result = query(&query_body)
            .bind(&app_id)
            .bind(&session_id)
            .bind(&connection_id)
            .bind(&app_id)
            .bind(&EntityType::App)
            .bind(&network)
            .execute(&mut **tx)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn create_new_connection_by_client(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        app_id: &String,
        session_id: &String,
        client_profile_id: i64,
        network: &String,
    ) -> Result<(), sqlx::Error> {
        let query_body = format!(
            "INSERT INTO {CONNECTION_EVENTS_TABLE_NAME} ({CONNECTION_EVENTS_KEYS_KEYS}) VALUES (DEFAULT, $1, $2, NULL, $3, $4, $5, NOW(), NULL)"
        );

        let query_result = query(&query_body)
            .bind(&app_id)
            .bind(&session_id)
            .bind(&client_profile_id)
            .bind(&EntityType::Client)
            .bind(&network)
            .execute(&mut **tx)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn close_app_connection(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        app_id: &String,
        connection_id: &String,
        session_id: &String,
    ) -> Result<(), sqlx::Error> {
        let query_body = format!(
            "UPDATE {CONNECTION_EVENTS_TABLE_NAME} SET disconnected_at = NOW() WHERE app_id = $1 AND session_id = $2 AND entity_type = $3 AND connection_id = $4"
        );

        let query_result = query(&query_body)
            .bind(&app_id)
            .bind(&session_id)
            .bind(&EntityType::App)
            .bind(&connection_id)
            .execute(&mut **tx)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn close_client_connection(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        app_id: &String,
        session_id: &String,
        client_profile_id: i64,
    ) -> Result<(), sqlx::Error> {
        let query_body = format!(
            "UPDATE {CONNECTION_EVENTS_TABLE_NAME} 
                SET disconnected_at = NOW() 
                WHERE app_id = $1 AND session_id = $2 AND entity_type = $3 AND entity_id = $4 AND disconnected_at IS NULL"
        );

        let query_result = query(&query_body)
            .bind(&app_id)
            .bind(&session_id)
            .bind(&EntityType::Client)
            .bind(&client_profile_id)
            .execute(&mut **tx)
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

    #[tokio::test]
    async fn test_connection_events() {
        let db = Db::connect_to_the_pool().await;
        db.truncate_all_tables().await.unwrap();

        let mut tx = db.connection_pool.begin().await.unwrap();

        let session_id = "session_id".to_string();
        let connection_id = "connection_id".to_string();
        let app_id = "app_id".to_string();
        let network = "network".to_string();

        // Create event by app
        db.create_new_connection_event_by_app(
            &mut tx,
            &session_id,
            &connection_id,
            &app_id,
            &network,
        )
        .await
        .unwrap();

        // Create event by client
        let client_profile_id = 1;
        db.create_new_connection_by_client(
            &mut tx,
            &app_id,
            &session_id,
            client_profile_id,
            &network,
        )
        .await
        .unwrap();

        tx.commit().await.unwrap();

        // Get events by session id
        let events_by_session_id = db
            .get_connection_events_by_session_id(&session_id)
            .await
            .unwrap();

        assert_eq!(events_by_session_id.len(), 2);

        // Get events by app id
        let events_by_app_id = db.get_connection_events_by_app(&app_id).await.unwrap();

        assert_eq!(events_by_app_id.len(), 1);

        // Get events by client profile id
        let events_by_client_profile_id = db
            .get_connection_events_by_client_profile_id(&client_profile_id.to_string())
            .await
            .unwrap();

        assert_eq!(events_by_client_profile_id.len(), 1);

        // Add another connection event by app with different connection id
        let connection_id = "connection_id_2".to_string();

        let mut tx = db.connection_pool.begin().await.unwrap();
        db.create_new_connection_event_by_app(
            &mut tx,
            &session_id,
            &connection_id,
            &app_id,
            &network,
        )
        .await
        .unwrap();

        tx.commit().await.unwrap();

        // Get events by app id
        let events_by_app_id = db.get_connection_events_by_app_id(&app_id).await.unwrap();

        assert_eq!(events_by_app_id.len(), 3);
    }
}
