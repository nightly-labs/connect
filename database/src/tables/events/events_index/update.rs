use crate::{
    db::Db,
    structs::{db_error::DbError, event_type::EventType},
    tables::events::events_index::table_struct::{EVENTS_KEYS, EVENTS_TABLE_NAME},
};
use sqlx::{query, Postgres, Row, Transaction};

impl Db {
    pub async fn create_new_event_entry(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        app_id: &String,
        event_type: &EventType,
    ) -> Result<i64, DbError> {
        let query_body = format!(
            "INSERT INTO {EVENTS_TABLE_NAME} ({EVENTS_KEYS}) VALUES (DEFAULT, $1, $2, DEFAULT) RETURNING event_id"
        );

        let query_result = query(&query_body)
            .bind(app_id)
            .bind(event_type)
            .fetch_one(&mut **tx)
            .await;

        match query_result {
            Ok(row) => Ok(row.get("event_id")),
            Err(e) => Err(e).map_err(|e| e.into()),
        }
    }
}
