use super::table_struct::{DbRegisteredApp, REGISTERED_APPS_TABLE_NAME};
use crate::structs::db_error::DbError;
use crate::tables::requests::table_struct::REQUESTS_TABLE_NAME;
use crate::{db::Db, tables::requests::table_struct::Request};
use sqlx::query_as;

impl Db {
    pub async fn get_registered_app_by_app_id(
        &self,
        app_id: &String,
    ) -> Result<Option<DbRegisteredApp>, DbError> {
        let query = format!("SELECT * FROM {REGISTERED_APPS_TABLE_NAME} WHERE app_id = $1");
        let typed_query = query_as::<_, DbRegisteredApp>(&query);

        return typed_query
            .bind(&app_id)
            .fetch_optional(&self.connection_pool)
            .await
            .map_err(|e| e.into());
    }

    pub async fn get_requests_by_app_id(&self, app_id: &String) -> Result<Vec<Request>, DbError> {
        let query = format!(
            "SELECT r.* FROM {REQUESTS_TABLE_NAME} r 
            INNER JOIN sessions s ON r.session_id = s.session_id 
            WHERE s.app_id = $1
            ORDER BY r.creation_timestamp DESC"
        );
        let typed_query = query_as::<_, Request>(&query);

        return typed_query
            .bind(&app_id)
            .fetch_all(&self.connection_pool)
            .await
            .map_err(|e| e.into());
    }

    pub async fn get_registered_app_by_app_name_and_team_id(
        &self,
        app_name: &String,
        team_id: &String,
    ) -> Result<Option<DbRegisteredApp>, DbError> {
        let query = format!(
            "SELECT * FROM {REGISTERED_APPS_TABLE_NAME} WHERE app_name = $1 AND team_id = $2"
        );
        let typed_query = query_as::<_, DbRegisteredApp>(&query);

        return typed_query
            .bind(&app_name)
            .bind(&team_id)
            .fetch_optional(&self.connection_pool)
            .await
            .map_err(|e| e.into());
    }
}
