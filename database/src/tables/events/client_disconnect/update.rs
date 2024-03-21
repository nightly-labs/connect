use crate::{
    db::Db,
    structs::db_error::DbError,
    tables::events::client_disconnect::table_struct::{
        EVENT_CLIENT_DISCONNECT_KEYS, EVENT_CLIENT_DISCONNECT_TABLE_NAME,
    },
};
use sqlx::{query, Postgres, Transaction};

impl Db {
    pub async fn create_new_event_client_disconnect(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        event_id: i64,
        client_id: &String,
        session_id: &String,
    ) -> Result<(), DbError> {
        let query_body = format!(
            "INSERT INTO {EVENT_CLIENT_DISCONNECT_TABLE_NAME} ({EVENT_CLIENT_DISCONNECT_KEYS}) VALUES ($1, $2, $3)"
        );

        let query_result = query(&query_body)
            .bind(event_id)
            .bind(client_id)
            .bind(session_id)
            .execute(&mut **tx)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e).map_err(|e| e.into()),
        }
    }
}
