use super::table_struct::User;
use crate::db::Db;
use crate::structs::db_error::DbError;
use crate::tables::users::table_struct::USERS_TABLE_NAME;
use sqlx::query_as;

impl Db {
    pub async fn get_user_by_user_id(&self, user_id: &String) -> Result<Option<User>, DbError> {
        let query = format!("SELECT * FROM {USERS_TABLE_NAME} WHERE user_id = $1");
        let typed_query = query_as::<_, User>(&query);

        return typed_query
            .bind(&user_id)
            .fetch_optional(&self.connection_pool)
            .await
            .map_err(|e| e.into());
    }

    pub async fn get_user_by_email(&self, email: &String) -> Result<Option<User>, DbError> {
        let query = format!("SELECT * FROM {USERS_TABLE_NAME} WHERE email = $1");
        let typed_query = query_as::<_, User>(&query);

        return typed_query
            .bind(&email)
            .fetch_optional(&self.connection_pool)
            .await
            .map_err(|e| e.into());
    }
}
