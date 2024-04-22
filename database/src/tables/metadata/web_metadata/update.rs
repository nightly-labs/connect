use crate::{
    db::Db,
    structs::db_error::DbError,
    tables::metadata::web_metadata::table_struct::{
        DbWebMetadata, WEB_METADATA_KEYS, WEB_METADATA_TABLE_NAME,
    },
};
use sqlx::{query, Transaction};

impl Db {
    pub async fn create_new_device_web_metadata_within_tx(
        &self,
        tx: &mut Transaction<'_, sqlx::Postgres>,
        metadata: &DbWebMetadata,
    ) -> Result<(), DbError> {
        let query_body =
            format!("INSERT INTO {WEB_METADATA_TABLE_NAME} ({WEB_METADATA_KEYS}) VALUES ($1, $2, $3, $4, $5)");

        let query_result = query(&query_body)
            .bind(&metadata.uuid)
            .bind(&metadata.browser)
            .bind(&metadata.browser_version)
            .bind(&metadata.os)
            .bind(&metadata.os_version)
            .execute(&mut **tx)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e).map_err(|e| e.into()),
        }
    }
}
