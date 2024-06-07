use super::table_struct::Network;
use crate::db::Db;
use crate::structs::db_error::DbError;
use crate::tables::networks::table_struct::NETWORKS_TABLE_NAME;
use sqlx::query_as;

impl Db {
    pub async fn get_all_networks(&self) -> Result<Vec<Network>, DbError> {
        let query = format!("SELECT * FROM {NETWORKS_TABLE_NAME}");
        let typed_query = query_as::<_, Network>(&query);

        return typed_query
            .fetch_all(&self.connection_pool)
            .await
            .map_err(|e| e.into());
    }
}
