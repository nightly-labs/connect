use super::table_struct::TeamInvite;
use crate::db::Db;
use crate::structs::db_error::DbError;
use crate::tables::team_invites::table_struct::TEAM_INVITES_TABLE_NAME;
use sqlx::query_as;

impl Db {
    pub async fn get_invites_by_team_id(
        &self,
        team_id: &String,
        active_invites: bool,
    ) -> Result<Vec<TeamInvite>, DbError> {
        let query = if active_invites {
            format!(
                "SELECT * FROM {TEAM_INVITES_TABLE_NAME} WHERE team_id = $1 AND accepted_at IS NULL AND cancelled_at IS NULL"
            )
        } else {
            format!("SELECT * FROM {TEAM_INVITES_TABLE_NAME} WHERE team_id = $1")
        };
        let typed_query = query_as::<_, TeamInvite>(&query);

        return typed_query
            .bind(&team_id)
            .fetch_all(&self.connection_pool)
            .await
            .map_err(|e| e.into());
    }

    pub async fn get_invites_by_user_email(
        &self,
        user_email: &String,
        active_invites: bool,
    ) -> Result<Vec<TeamInvite>, DbError> {
        let query = if active_invites {
            format!(
                "SELECT * FROM {TEAM_INVITES_TABLE_NAME} WHERE user_email = $1 AND accepted_at IS NULL AND cancelled_at IS NULL"
            )
        } else {
            format!("SELECT * FROM {TEAM_INVITES_TABLE_NAME} WHERE user_email = $1")
        };
        let typed_query = query_as::<_, TeamInvite>(&query);

        return typed_query
            .bind(&user_email)
            .fetch_all(&self.connection_pool)
            .await
            .map_err(|e| e.into());
    }
}
