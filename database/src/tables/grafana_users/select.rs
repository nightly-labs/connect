use super::table_struct::GrafanaUser;
use crate::db::Db;
use crate::tables::grafana_users::table_struct::GRAFANA_USERS_TABLE_NAME;
use sqlx::query_as;

impl Db {
    pub async fn get_user_by_user_id(&self, user_id: &String) -> Result<GrafanaUser, sqlx::Error> {
        let query = format!("SELECT * FROM {GRAFANA_USERS_TABLE_NAME} WHERE user_id = $1");
        let typed_query = query_as::<_, GrafanaUser>(&query);

        return typed_query
            .bind(&user_id)
            .fetch_one(&self.connection_pool)
            .await;
    }

    pub async fn get_user_by_email(&self, email: &String) -> Result<GrafanaUser, sqlx::Error> {
        let query = format!("SELECT * FROM {GRAFANA_USERS_TABLE_NAME} WHERE email = $1");
        let typed_query = query_as::<_, GrafanaUser>(&query);

        return typed_query
            .bind(&email)
            .fetch_one(&self.connection_pool)
            .await;
    }
}
