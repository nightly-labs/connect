use super::table_struct::ClientProfile;
use crate::db::Db;
use crate::tables::client_profiles::table_struct::CLIENT_PROFILES_TABLE_NAME;
use sqlx::query_as;

impl Db {
    pub async fn get_profile_by_profile_id(
        &self,
        client_profile_id: i64,
    ) -> Result<ClientProfile, sqlx::Error> {
        let query =
            format!("SELECT * FROM {CLIENT_PROFILES_TABLE_NAME} WHERE client_profile_id = $1");
        let typed_query = query_as::<_, ClientProfile>(&query);

        return typed_query
            .bind(&client_profile_id)
            .fetch_one(&self.connection_pool)
            .await;
    }
}
