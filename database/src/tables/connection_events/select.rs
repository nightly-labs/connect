use super::table_struct::ConnectionEvent;
use crate::db::Db;
use crate::structs::entity_type::EntityType;
use crate::tables::connection_events::table_struct::CONNECTION_EVENTS_TABLE_NAME;
use sqlx::query_as;

impl Db {
    pub async fn get_connection_events_by_session_id(
        &self,
        session_id: &String,
    ) -> Result<Vec<ConnectionEvent>, sqlx::Error> {
        let query = format!("SELECT * FROM {CONNECTION_EVENTS_TABLE_NAME} WHERE session_id = $1");
        let typed_query = query_as::<_, ConnectionEvent>(&query);

        return typed_query
            .bind(&session_id)
            .fetch_all(&self.connection_pool)
            .await;
    }

    pub async fn get_connection_events_by_client_profile_id(
        &self,
        client_profile_id: &String,
    ) -> Result<Vec<ConnectionEvent>, sqlx::Error> {
        let query = format!(
            "SELECT * FROM {CONNECTION_EVENTS_TABLE_NAME} WHERE entity_id = $1 AND entity_type = $2"
        );
        let typed_query = query_as::<_, ConnectionEvent>(&query);

        return typed_query
            .bind(&client_profile_id)
            .bind(&EntityType::Client)
            .fetch_all(&self.connection_pool)
            .await;
    }

    pub async fn get_connection_events_by_app_id(
        &self,
        app_id: &String,
    ) -> Result<Vec<ConnectionEvent>, sqlx::Error> {
        let query = format!("SELECT * FROM {CONNECTION_EVENTS_TABLE_NAME} WHERE app_id = $1");
        let typed_query = query_as::<_, ConnectionEvent>(&query);

        return typed_query
            .bind(&app_id)
            .fetch_all(&self.connection_pool)
            .await;
    }

    pub async fn get_connection_events_by_app(
        &self,
        app_id: &String,
    ) -> Result<Vec<ConnectionEvent>, sqlx::Error> {
        let query = format!("SELECT * FROM {CONNECTION_EVENTS_TABLE_NAME} WHERE entity_id = $1 AND entity_type = $2");
        let typed_query = query_as::<_, ConnectionEvent>(&query);

        return typed_query
            .bind(&app_id)
            .bind(&EntityType::App)
            .fetch_all(&self.connection_pool)
            .await;
    }
}
