use super::table_struct::PublicKey;
use crate::db::Db;
use crate::tables::public_keys::table_struct::PUBLIC_KEYS_TABLE_NAME;
use sqlx::query_as;

impl Db {
    pub async fn get_public_key(&self, public_key: &String) -> Result<PublicKey, sqlx::Error> {
        let query = format!("SELECT * FROM {PUBLIC_KEYS_TABLE_NAME} WHERE public_key = $1");
        let typed_query = query_as::<_, PublicKey>(&query);

        return typed_query
            .bind(&public_key)
            .fetch_one(&self.connection_pool)
            .await;
    }

    pub async fn get_public_keys_associated_with_client_profile_id(
        &self,
        client_profile_id: i64,
    ) -> Result<Vec<PublicKey>, sqlx::Error> {
        let query = format!("SELECT * FROM {PUBLIC_KEYS_TABLE_NAME} WHERE client_profile_id = $1");
        let typed_query = query_as::<_, PublicKey>(&query);

        return typed_query
            .bind(&client_profile_id)
            .fetch_all(&self.connection_pool)
            .await;
    }
}
