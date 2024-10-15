use super::table_struct::ConnectionEvent;
use crate::db::Db;
use crate::structs::db_error::DbError;
use crate::structs::entity_type::EntityType;
use crate::tables::connection_events::table_struct::CONNECTION_EVENTS_TABLE_NAME;
use sqlx::query_as;

impl Db {
    pub async fn get_last_connection_attempt_by_client(
        &self,
        app_id: &String,
        session_id: &String,
    ) -> Result<Option<ConnectionEvent>, DbError> {
        let query_body = format!(
            "SELECT * FROM {CONNECTION_EVENTS_TABLE_NAME} 
                WHERE app_id = $1 AND session_id = $2 AND entity_type = $3 AND success = false
                ORDER BY connected_at DESC LIMIT 1"
        );

        let query_result = query_as::<_, ConnectionEvent>(&query_body)
            .bind(app_id)
            .bind(session_id)
            .bind(EntityType::Client)
            .bind(true)
            .fetch_optional(&self.connection_pool)
            .await;

        match query_result {
            Ok(result) => Ok(result),
            Err(e) => Err(e.into()),
        }
    }
}
