use super::table_struct::IpAddresses;
use crate::db::Db;
use crate::structs::db_error::DbError;
use crate::tables::ip_addresses::table_struct::IP_ADDRESSES_TABLE_NAME;
use sqlx::query_as;

impl Db {
    pub async fn get_ip_address(&self, ip_addr: &String) -> Result<Option<IpAddresses>, DbError> {
        let query = format!("SELECT * FROM {IP_ADDRESSES_TABLE_NAME} WHERE ip_addr = $1");
        let typed_query = query_as::<_, IpAddresses>(&query);

        return typed_query
            .bind(ip_addr)
            .fetch_optional(&self.connection_pool)
            .await
            .map_err(|e| e.into());
    }
}
