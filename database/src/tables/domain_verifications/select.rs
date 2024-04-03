use crate::{
    db::Db,
    structs::db_error::DbError,
    tables::domain_verifications::table_struct::{
        DomainVerification, DOMAIN_VERIFICATIONS_TABLE_NAME,
    },
};
use sqlx::query_as;

impl Db {
    pub async fn get_domain_verifications_by_app_id(
        &self,
        app_id: &String,
    ) -> Result<Vec<DomainVerification>, DbError> {
        let query = format!("SELECT * FROM {DOMAIN_VERIFICATIONS_TABLE_NAME} WHERE app_id = $1 ORDER BY created_at DESC");
        let typed_query = query_as::<_, DomainVerification>(&query);

        return typed_query
            .bind(&app_id)
            .fetch_all(&self.connection_pool)
            .await
            .map_err(|e| e.into());
    }
}
