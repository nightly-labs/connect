use crate::tables::events::app_disconnect::table_struct::{
    EVENT_APP_DISCONNECT_KEYS, EVENT_APP_DISCONNECT_TABLE_NAME,
};
use crate::{db::Db, structs::db_error::DbError};
use sqlx::Transaction;
use sqlx::{query, Postgres};

impl Db {
    pub async fn create_new_event_app_disconnect(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        session_id: &String,
    ) -> Result<(), DbError> {
        let query_body = format!(
            "INSERT INTO {EVENT_APP_DISCONNECT_TABLE_NAME} ({EVENT_APP_DISCONNECT_KEYS}) VALUES (DEFAULT, $1)"
        );

        let query_result = query(&query_body).bind(session_id).execute(&mut **tx).await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e).map_err(|e| e.into()),
        }
    }
}
