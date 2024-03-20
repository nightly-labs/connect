use crate::tables::events::app_connect::table_struct::EVENT_APP_CONNECT_KEYS;
use crate::tables::events::app_connect::table_struct::EVENT_APP_CONNECT_TABLE_NAME;
use crate::{db::Db, structs::db_error::DbError};
use sqlx::Transaction;
use sqlx::{query, Postgres};

impl Db {
    pub async fn create_new_event_app_connect(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        session_id: &String,
        device_metadata: &String,
        lang: &String,
        timezone: &String,
        new_session: bool,
    ) -> Result<(), DbError> {
        let query_body = format!(
            "INSERT INTO {EVENT_APP_CONNECT_TABLE_NAME} ({EVENT_APP_CONNECT_KEYS}) VALUES (DEFAULT, $1, $2, $3, $4, $5)"
        );

        let query_result = query(&query_body)
            .bind(session_id)
            .bind(device_metadata)
            .bind(lang)
            .bind(timezone)
            .bind(new_session)
            .execute(&mut **tx)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e).map_err(|e| e.into()),
        }
    }
}
