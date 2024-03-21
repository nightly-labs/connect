use crate::{
    db::Db,
    structs::db_error::DbError,
    tables::events::client_connect::table_struct::{
        EVENT_CLIENT_CONNECT_KEYS, EVENT_CLIENT_CONNECT_TABLE_NAME,
    },
};
use sqlx::{query, Postgres, Transaction};

impl Db {
    pub async fn create_new_event_client_connect(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        event_id: i64,
        client_id: &String,
        session_id: &String,
        wallet_name: &String,
        wallet_type: &String,
        session_type: &String,
    ) -> Result<(), DbError> {
        let query_body = format!(
            "INSERT INTO {EVENT_CLIENT_CONNECT_TABLE_NAME} ({EVENT_CLIENT_CONNECT_KEYS}) VALUES ($1, $2, $3, NULL, $4, $5, $6, false)"
        );

        let query_result = query(&query_body)
            .bind(event_id)
            .bind(client_id)
            .bind(session_id)
            .bind(wallet_name)
            .bind(wallet_type)
            .bind(session_type)
            .execute(&mut **tx)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e).map_err(|e| e.into()),
        }
    }

    pub async fn update_event_client_connect_success(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        client_id: &String,
        session_id: &String,
        success: bool,
        new_addresses: Vec<String>,
    ) -> Result<(), DbError> {
        let query_body = format!(
            "UPDATE {EVENT_CLIENT_CONNECT_TABLE_NAME} SET success = $1, addresses = $2 WHERE client_id = $3 AND session_id = $4 AND success = false"
        );

        let query_result = query(&query_body)
            .bind(success)
            .bind(&new_addresses)
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
