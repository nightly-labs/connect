use super::table_struct::{Request, REQUESTS_TABLE_NAME};
use crate::{db::Db, structs::db_error::DbError};
use sqlx::query_as;

impl Db {
    pub async fn get_requests_by_session_id(
        &self,
        session_id: &String,
    ) -> Result<Vec<Request>, DbError> {
        let query = format!("SELECT * FROM {REQUESTS_TABLE_NAME} WHERE session_id = $1 ORDER BY creation_timestamp DESC");
        let typed_query = query_as::<_, Request>(&query);

        return typed_query
            .bind(&session_id)
            .fetch_all(&self.connection_pool)
            .await
            .map_err(|e| e.into());
    }

    pub async fn get_request_by_request_id(
        &self,
        request_id: &String,
    ) -> Result<Option<Request>, DbError> {
        let query = format!("SELECT * FROM {REQUESTS_TABLE_NAME} WHERE request_id = $1");
        let typed_query = query_as::<_, Request>(&query);

        return typed_query
            .bind(&request_id)
            .fetch_optional(&self.connection_pool)
            .await
            .map_err(|e| e.into());
    }
}
