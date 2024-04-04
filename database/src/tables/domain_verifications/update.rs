use super::table_struct::{DOMAIN_VERIFICATIONS_KEYS, DOMAIN_VERIFICATIONS_TABLE_NAME};
use crate::db::Db;
use crate::structs::db_error::DbError;
use crate::tables::utils::get_current_datetime;
use sqlx::query;

impl Db {
    pub async fn create_new_domain_verification_entry(
        &self,
        domain_name: &String,
        app_id: &String,
        code: &String,
    ) -> Result<(), DbError> {
        let query_body = format!(
            "INSERT INTO {DOMAIN_VERIFICATIONS_TABLE_NAME} ({DOMAIN_VERIFICATIONS_KEYS}) VALUES ($1, $2, $3, $4, NULL)"
        );

        let query_result = query(&query_body)
            .bind(&domain_name)
            .bind(&app_id)
            .bind(&code)
            .bind(&get_current_datetime())
            .execute(&self.connection_pool)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e).map_err(|e| e.into()),
        }
    }

    pub async fn finish_domain_verification(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        domain_name: &String,
    ) -> Result<(), DbError> {
        let query_body = format!(
            "UPDATE {DOMAIN_VERIFICATIONS_TABLE_NAME} SET finished_at = $1 WHERE domain_name = $2"
        );

        let query_result = query(&query_body)
            .bind(&get_current_datetime())
            .bind(&domain_name)
            .execute(&mut **tx)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e).map_err(|e| e.into()),
        }
    }
}
