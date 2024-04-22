use crate::{
    db::Db,
    structs::db_error::DbError,
    tables::metadata::mobile_metadata::table_struct::{
        DbMobileMetadata, MOBILE_METADATA_KEYS, MOBILE_METADATA_TABLE_NAME,
    },
};
use sqlx::{query, Transaction};

impl Db {
    pub async fn create_new_device_mobile_metadata_within_tx(
        &self,
        tx: &mut Transaction<'_, sqlx::Postgres>,
        metadata: &DbMobileMetadata,
    ) -> Result<(), DbError> {
        let query_body = format!(
            "INSERT INTO {MOBILE_METADATA_TABLE_NAME} ({MOBILE_METADATA_KEYS}) VALUES ($1, $2, $3)"
        );

        let query_result = query(&query_body)
            .bind(&metadata.uuid)
            .bind(&metadata.system_type)
            .bind(&metadata.system_version)
            .execute(&mut **tx)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e).map_err(|e| e.into()),
        }
    }
}
