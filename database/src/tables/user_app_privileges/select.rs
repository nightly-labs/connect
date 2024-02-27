use super::table_struct::UserAppPrivilege;
use crate::db::Db;
use crate::tables::user_app_privileges::table_struct::USER_APP_PRIVILEGES_TABLE_NAME;
use sqlx::query_as;

impl Db {
    pub async fn get_privilege_by_user_id_and_app_id(
        &self,
        user_id: &String,
        app_id: &String,
    ) -> Result<UserAppPrivilege, sqlx::Error> {
        let query = format!(
            "SELECT * FROM {USER_APP_PRIVILEGES_TABLE_NAME} WHERE user_id = $1 AND app_id = $2"
        );
        let typed_query = query_as::<_, UserAppPrivilege>(&query);

        return typed_query
            .bind(&user_id)
            .bind(&app_id)
            .fetch_one(&self.connection_pool)
            .await;
    }

    pub async fn get_privileges_by_user_id(
        &self,
        user_id: &String,
    ) -> Result<Vec<UserAppPrivilege>, sqlx::Error> {
        let query = format!("SELECT * FROM {USER_APP_PRIVILEGES_TABLE_NAME} WHERE user_id = $1");
        let typed_query = query_as::<_, UserAppPrivilege>(&query);

        return typed_query
            .bind(&user_id)
            .fetch_all(&self.connection_pool)
            .await;
    }

    pub async fn get_privileges_by_app_id(
        &self,
        app_id: &String,
    ) -> Result<Vec<UserAppPrivilege>, sqlx::Error> {
        let query = format!("SELECT * FROM {USER_APP_PRIVILEGES_TABLE_NAME} WHERE app_id = $1");
        let typed_query = query_as::<_, UserAppPrivilege>(&query);

        return typed_query
            .bind(&app_id)
            .fetch_all(&self.connection_pool)
            .await;
    }
}
