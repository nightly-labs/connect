use crate::structs::request_status::RequestStatus;
use crate::{
    db::Db,
    structs::db_error::DbError,
    tables::events::sign_message::table_struct::{
        EVENT_SIGN_MESSAGE_KEYS, EVENT_SIGN_MESSAGE_TABLE_NAME,
    },
};
use sqlx::{query, Postgres, Transaction};

impl Db {
    pub async fn create_new_event_sign_message(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        event_id: i64,
        session_id: &String,
        request_id: &String,
        network_id: &String,
    ) -> Result<(), DbError> {
        let query_body = format!(
            "INSERT INTO {EVENT_SIGN_MESSAGE_TABLE_NAME} ({EVENT_SIGN_MESSAGE_KEYS}) VALUES ($1, $2, $3, $4, $5)"
        );

        let query_result = query(&query_body)
            .bind(event_id)
            .bind(session_id)
            .bind(request_id)
            .bind(RequestStatus::Pending)
            .bind(network_id)
            .execute(&mut **tx)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e).map_err(|e| e.into()),
        }
    }

    pub async fn update_event_sign_message(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        request_id: &String,
        request_status: RequestStatus,
    ) -> Result<(), DbError> {
        let query_body = format!(
            "UPDATE {EVENT_SIGN_MESSAGE_TABLE_NAME} SET request_status = $1, WHERE request_id = $2"
        );

        let query_result = query(&query_body)
            .bind(request_status)
            .bind(request_id)
            .execute(&mut **tx)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e).map_err(|e| e.into()),
        }
    }
}
