use super::table_struct::TeamInvite;
use crate::db::Db;
use crate::structs::db_error::DbError;
use crate::tables::team_invites::table_struct::TEAM_INVITES_TABLE_NAME;

impl Db {
    pub async fn get_invites_by_team_id(
        &self,
        team_id: &String,
        active_invites: bool,
    ) -> Result<Vec<TeamInvite>, DbError> {
        let additional_filter = if active_invites {
            "AND ti.accepted_at IS NULL AND ti.cancelled_at IS NULL"
        } else {
            ""
        };

        let query = format!(
            "SELECT ti.invite_id, ti.team_id, ti.user_email, ti.created_at,
             ti.accepted_at, ti.cancelled_at, t.team_name, gu.email AS admin_email
             FROM {TEAM_INVITES_TABLE_NAME} ti
             INNER JOIN team t ON ti.team_id = t.team_id
             INNER JOIN users gu ON t.team_admin_id = gu.user_id
             WHERE ti.team_id = $1 {additional_filter}
             ORDER BY ti.created_at DESC",
        );

        let typed_query = sqlx::query_as::<_, TeamInvite>(&query).bind(team_id);

        return typed_query
            .fetch_all(&self.connection_pool)
            .await
            .map_err(|e| e.into());
    }

    pub async fn get_invites_by_user_email(
        &self,
        user_email: &String,
        active_invites: bool,
    ) -> Result<Vec<TeamInvite>, DbError> {
        let additional_filter = if active_invites {
            "AND ti.accepted_at IS NULL AND ti.cancelled_at IS NULL"
        } else {
            ""
        };

        let query = format!(
            "SELECT ti.invite_id, ti.team_id, ti.user_email, ti.created_at, \
             ti.accepted_at, ti.cancelled_at, t.team_name, gu.email AS admin_email \
             FROM {TEAM_INVITES_TABLE_NAME} ti \
             INNER JOIN team t ON ti.team_id = t.team_id \
             INNER JOIN users gu ON t.team_admin_id = gu.user_id \
             WHERE ti.user_email = $1 {additional_filter} \
             ORDER BY ti.created_at DESC",
            TEAM_INVITES_TABLE_NAME = TEAM_INVITES_TABLE_NAME
        );

        let typed_query = sqlx::query_as::<_, TeamInvite>(&query).bind(user_email);

        return typed_query
            .fetch_all(&self.connection_pool)
            .await
            .map_err(|e| e.into());
    }
}
