use super::table_struct::Team;
use crate::tables::registered_app::table_struct::REGISTERED_APPS_TABLE_NAME;
use crate::tables::team::table_struct::TEAM_TABLE_NAME;
use crate::{db::Db, tables::registered_app::table_struct::DbRegisteredApp};
use sqlx::{query_as, Transaction};

impl Db {
    pub async fn get_team_by_team_id(
        &self,
        tx: Option<&mut Transaction<'_, sqlx::Postgres>>,
        team_id: &String,
    ) -> Result<Option<Team>, sqlx::Error> {
        let query = format!("SELECT * FROM {TEAM_TABLE_NAME} WHERE team_id = $1");
        let typed_query = query_as::<_, Team>(&query);

        match tx {
            Some(tx) => return typed_query.bind(&team_id).fetch_optional(&mut **tx).await,
            None => {
                return typed_query
                    .bind(&team_id)
                    .fetch_optional(&self.connection_pool)
                    .await
            }
        }
    }

    pub async fn get_registered_apps_by_team_id(
        &self,
        team_id: &String,
    ) -> Result<Vec<DbRegisteredApp>, sqlx::Error> {
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
            .await;
    }

    pub async fn get_team_by_admin_id(
        &self,
        admin_id: &String,
    ) -> Result<Option<Team>, sqlx::Error> {
        let query = format!("SELECT * FROM {TEAM_TABLE_NAME} WHERE team_admin_id = $1");
        let typed_query = query_as::<_, Team>(&query);

        return typed_query
            .bind(&admin_id)
            .fetch_optional(&self.connection_pool)
            .await;
    }
}
