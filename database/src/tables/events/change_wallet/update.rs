use crate::structs::request_status::RequestStatus;
use crate::{
    db::Db,
    structs::db_error::DbError,
    tables::events::change_wallet::table_struct::{
        EVENT_CHANGE_WALLET_KEYS, EVENT_CHANGE_WALLET_TABLE_NAME,
    },
};
use sqlx::{query, Postgres, Transaction};

impl Db {
    pub async fn create_new_event_change_wallet(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        event_id: &i64,
        session_id: &String,
        request_id: &String,
        network_id: &String,
        wallet_name: &String,
        wallet_type: &String,
        old_wallet_address: &String,
    ) -> Result<(), DbError> {
        let query_body = format!(
            "INSERT INTO {EVENT_CHANGE_WALLET_TABLE_NAME} ({EVENT_CHANGE_WALLET_KEYS}) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NULL)"
        );

        let query_result = query(&query_body)
            .bind(event_id)
            .bind(session_id)
            .bind(request_id)
            .bind(RequestStatus::Pending)
            .bind(network_id)
            .bind(wallet_name)
            .bind(wallet_type)
            .bind(old_wallet_address)
            .execute(&mut **tx)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e).map_err(|e| e.into()),
        }
    }

    pub async fn update_event_change_wallet(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        request_id: &String,
        request_status: RequestStatus,
        new_wallet_address: Option<String>,
    ) -> Result<(), DbError> {
        let query_body = format!(
            "UPDATE {EVENT_CHANGE_WALLET_TABLE_NAME} SET request_status = $1, new_wallet_address = $2 WHERE request_id = $3",
        );

        let query_result = query(&query_body)
            .bind(request_status)
            .bind(new_wallet_address)
            .bind(request_id)
            .execute(&mut **tx)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e).map_err(|e| e.into()),
        }
    }
}
