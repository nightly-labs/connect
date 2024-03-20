use super::table_struct::{DbNcSession, SESSIONS_TABLE_NAME};
use crate::db::Db;
use crate::structs::db_error::DbError;
use sqlx::query_as;

impl Db {
    pub async fn get_sessions_by_app_id(
        &self,
        app_id: &String,
    ) -> Result<Vec<DbNcSession>, DbError> {
        let query = format!("SELECT * FROM {SESSIONS_TABLE_NAME} WHERE app_id = $1 ORDER BY session_open_timestamp DESC");
        let typed_query = query_as::<_, DbNcSession>(&query);

        return typed_query
            .bind(&app_id)
            .fetch_all(&self.connection_pool)
            .await
            .map_err(|e| e.into());
    }

    pub async fn get_session_by_session_id(
        &self,
        session_id: &String,
    ) -> Result<Option<DbNcSession>, DbError> {
        let query = format!("SELECT * FROM {SESSIONS_TABLE_NAME} WHERE session_id = $1");
        let typed_query = query_as::<_, DbNcSession>(&query);

        return typed_query
            .bind(&session_id)
            .fetch_optional(&self.connection_pool)
            .await
            .map_err(|e| e.into());
    }
}
