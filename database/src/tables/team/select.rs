use super::table_struct::Team;
use crate::db::Db;
use crate::tables::team::table_struct::TEAM_TABLE_NAME;
use sqlx::{query_as, Transaction};

impl Db {
    pub async fn get_team_by_team_id(
        &self,
        tx: Option<&mut Transaction<'_, sqlx::Postgres>>,
        team_id: &String,
    ) -> Result<Team, sqlx::Error> {
        let query = format!("SELECT * FROM {TEAM_TABLE_NAME} WHERE team_id = $1");
        let typed_query = query_as::<_, Team>(&query);

        match tx {
            Some(tx) => return typed_query.bind(&team_id).fetch_one(&mut **tx).await,
            None => {
                return typed_query
                    .bind(&team_id)
                    .fetch_one(&self.connection_pool)
                    .await
            }
        }
    }
}
