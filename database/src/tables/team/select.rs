use super::table_struct::Team;
use crate::structs::db_error::DbError;
use crate::tables::registered_app::table_struct::REGISTERED_APPS_TABLE_NAME;
use crate::tables::team::table_struct::TEAM_TABLE_NAME;
use crate::{db::Db, tables::registered_app::table_struct::DbRegisteredApp};
use sqlx::{query_as, Transaction};

impl Db {
    pub async fn get_team_by_team_id(
        &self,
        tx: Option<&mut Transaction<'_, sqlx::Postgres>>,
        team_id: &String,
    ) -> Result<Option<Team>, DbError> {
        let query = format!("SELECT * FROM {TEAM_TABLE_NAME} WHERE team_id = $1");
        let typed_query = query_as::<_, Team>(&query);

        match tx {
            Some(tx) => {
                return typed_query
                    .bind(&team_id)
                    .fetch_optional(&mut **tx)
                    .await
                    .map_err(|e| e.into())
            }
            None => {
                return typed_query
                    .bind(&team_id)
                    .fetch_optional(&self.connection_pool)
                    .await
                    .map_err(|e| e.into())
            }
        }
    }

    pub async fn get_registered_apps_by_team_id(
        &self,
        team_id: &String,
    ) -> Result<Vec<DbRegisteredApp>, DbError> {
        let query = format!(
            "SELECT r.* FROM {REGISTERED_APPS_TABLE_NAME} r 
            INNER JOIN team t ON r.team_id = t.team_id 
            WHERE t.team_id = $1
            ORDER BY t.registration_timestamp DESC"
        );
        let typed_query = query_as::<_, DbRegisteredApp>(&query);

        return typed_query
            .bind(&team_id)
            .fetch_all(&self.connection_pool)
            .await
            .map_err(|e| e.into());
    }

    pub async fn get_user_created_teams_without_personal(
        &self,
        admin_id: &String,
    ) -> Result<Vec<Team>, DbError> {
        let query = format!(
            "SELECT * FROM {TEAM_TABLE_NAME} WHERE team_admin_id = $1 AND personal = false"
        );
        let typed_query = query_as::<_, Team>(&query);

        return typed_query
            .bind(&admin_id)
            .fetch_all(&self.connection_pool)
            .await
            .map_err(|e| e.into());
    }

    pub async fn get_personal_team_by_admin_id(
        &self,
        admin_id: &String,
    ) -> Result<Option<Team>, DbError> {
        let query =
            format!("SELECT * FROM {TEAM_TABLE_NAME} WHERE team_admin_id = $1 AND personal = true");
        let typed_query = query_as::<_, Team>(&query);

        return typed_query
            .bind(&admin_id)
            .fetch_optional(&self.connection_pool)
            .await
            .map_err(|e| e.into());
    }

    pub async fn get_team_by_team_name_and_admin_id(
        &self,
        team_name: &String,
        team_id: &String,
    ) -> Result<Option<Team>, DbError> {
        let query =
            format!("SELECT * FROM {TEAM_TABLE_NAME} WHERE team_name = $1 AND team_id = $2");
        let typed_query = query_as::<_, Team>(&query);

        return typed_query
            .bind(&team_name)
            .bind(&team_id)
            .fetch_optional(&self.connection_pool)
            .await
            .map_err(|e| e.into());
    }

    pub async fn get_team_by_admin_id(&self, admin_id: &String) -> Result<Option<Team>, DbError> {
        let query = format!("SELECT * FROM {TEAM_TABLE_NAME} WHERE team_admin_id = $1");
        let typed_query = query_as::<_, Team>(&query);

        return typed_query
            .bind(&admin_id)
            .fetch_optional(&self.connection_pool)
            .await
            .map_err(|e| e.into());
    }
}
