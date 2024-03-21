use crate::structs::request_status::RequestStatus;
use crate::{
    db::Db,
    structs::db_error::DbError,
    tables::events::change_network::table_struct::{
        EVENT_CHANGE_NETWORK_KEYS, EVENT_CHANGE_NETWORK_TABLE_NAME,
    },
};
use sqlx::{query, Postgres, Transaction};

impl Db {
    pub async fn create_new_event_change_network(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        event_id: i64,
        session_id: &String,
        request_id: &String,
        old_network: &String,
    ) -> Result<(), DbError> {
        let query_body = format!(
            "INSERT INTO {EVENT_CHANGE_NETWORK_TABLE_NAME} ({EVENT_CHANGE_NETWORK_KEYS}) VALUES ($1, $2, $3, $4, $5, NULL)"
        );

        let query_result = query(&query_body)
            .bind(event_id)
            .bind(session_id)
            .bind(request_id)
            .bind(RequestStatus::Pending)
            .bind(old_network)
            .execute(&mut **tx)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e).map_err(|e| e.into()),
        }
    }

    pub async fn update_event_change_network(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        request_id: &String,
        request_status: RequestStatus,
        new_network: &Option<String>,
    ) -> Result<(), DbError> {
        let query_body = format!(
            "UPDATE {EVENT_CHANGE_NETWORK_TABLE_NAME} SET request_status = $1, new_network = $2 WHERE request_id = $3",
        );

        let query_result = query(&query_body)
            .bind(request_status)
            .bind(new_network)
            .bind(request_id)
            .execute(&mut **tx)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e).map_err(|e| e.into()),
        }
    }
}
