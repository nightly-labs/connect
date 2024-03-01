use crate::db::Db;
use crate::tables::session_public_keys::table_struct::SessionPublicKey;
use crate::tables::session_public_keys::table_struct::SESSION_PUBLIC_KEYS_TABLE_NAME;
use sqlx::query_as;

impl Db {
    pub async fn get_session_public_keys(
        &self,
        session_id: &String,
    ) -> Result<Vec<SessionPublicKey>, sqlx::Error> {
        let query = format!("SELECT * FROM {SESSION_PUBLIC_KEYS_TABLE_NAME} WHERE session_id = $1");
        let typed_query = query_as::<_, SessionPublicKey>(&query);

        return typed_query
            .bind(&session_id)
            .fetch_all(&self.connection_pool)
            .await;
    }
}
