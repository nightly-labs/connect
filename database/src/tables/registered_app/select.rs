use super::table_struct::{RegisteredApp, REGISTERED_APPS_TABLE_NAME};
use crate::db::Db;
use sqlx::query_as;

impl Db {
    pub async fn get_registered_app_by_app_id(
        &self,
        app_id: &String,
    ) -> Result<RegisteredApp, sqlx::Error> {
        let query = format!("SELECT * FROM {REGISTERED_APPS_TABLE_NAME} WHERE app_id = $1");
        let typed_query = query_as::<_, RegisteredApp>(&query);

        return typed_query
            .bind(&app_id)
            .fetch_one(&self.connection_pool)
            .await;
    }
}
