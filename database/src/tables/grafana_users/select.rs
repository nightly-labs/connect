use super::table_struct::GrafanaUser;
use crate::db::Db;
use crate::tables::grafana_users::table_struct::GRAFANA_USERS_TABLE_NAME;
use sqlx::query_as;

impl Db {
    pub async fn get_user_by_user_name(
        &self,
        user_name: &String,
    ) -> Result<GrafanaUser, sqlx::Error> {
        let query = format!("SELECT * FROM {GRAFANA_USERS_TABLE_NAME} WHERE name = $1");
        let typed_query = query_as::<_, GrafanaUser>(&query);

        return typed_query
            .bind(&user_name)
            .fetch_one(&self.connection_pool)
            .await;
    }
}
