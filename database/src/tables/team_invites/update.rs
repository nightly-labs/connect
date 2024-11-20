use super::table_struct::{TEAM_INVITES_KEYS, TEAM_INVITES_TABLE_NAME};
use crate::db::Db;
use crate::structs::db_error::DbError;
use crate::tables::{team::table_struct::TEAM_TABLE_NAME, utils::get_current_datetime};
use sqlx::{query, Transaction};

impl Db {
    pub async fn create_new_team_invite(
        &self,
        team_id: &String,
        user_email: &String,
    ) -> Result<(), DbError> {
        let query_body = format!(
            "INSERT INTO {TEAM_INVITES_TABLE_NAME} ({TEAM_INVITES_KEYS}) VALUES (DEFAULT, $1, $2, $3, NULL, NULL)"
        );

        let query_result = query(&query_body)
            .bind(&team_id)
            .bind(&user_email)
            .bind(&get_current_datetime())
            .execute(&self.connection_pool)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e).map_err(|e| e.into()),
        }
    }

    pub async fn accept_team_invite(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        team_id: &String,
        user_email: &String,
    ) -> Result<(), DbError> {
        let query_body = format!(
            "UPDATE {TEAM_INVITES_TABLE_NAME} SET accepted_at = $1 WHERE team_id = $2 AND user_email = $3 AND accepted_at IS NULL AND cancelled_at IS NULL"
        );

        let query_result = query(&query_body)
            .bind(&get_current_datetime())
            .bind(&team_id)
            .bind(&user_email)
            .execute(&mut **tx)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e).map_err(|e| e.into()),
        }
    }

    pub async fn cancel_team_invite(
        &self,
        team_id: &String,
        user_email: &String,
    ) -> Result<(), DbError> {
        let query_body = format!(
            "UPDATE {TEAM_INVITES_TABLE_NAME} SET cancelled_at = $1 WHERE team_id = $2 AND user_email = $3 AND accepted_at IS NULL AND cancelled_at IS NULL"
        );

        let query_result = query(&query_body)
            .bind(&get_current_datetime())
            .bind(&team_id)
            .bind(&user_email)
            .execute(&self.connection_pool)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e).map_err(|e| e.into()),
        }
    }

    pub async fn cancel_all_team_invites(
        &self,
        tx: &mut Transaction<'_, sqlx::Postgres>,
        team_id: &String,
    ) -> Result<(), DbError> {
        let query_body = format!(
            "UPDATE {TEAM_INVITES_TABLE_NAME} SET cancelled_at = $1 WHERE team_id = $2 AND accepted_at IS NULL AND cancelled_at IS NULL"
        );

        let query_result = query(&query_body)
            .bind(&get_current_datetime())
            .bind(&team_id)
            .execute(&mut **tx)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e).map_err(|e| e.into()),
        }
    }

    pub async fn cancel_all_team_invites_containing_email(
        &self,
        tx: &mut Transaction<'_, sqlx::Postgres>,
        user_email: &String,
        user_id: &String,
    ) -> Result<(), DbError> {
        let query_body = format!(
            "UPDATE {TEAM_INVITES_TABLE_NAME} SET cancelled_at = $1 WHERE (user_email = $2 OR team_id IN (SELECT team_id FROM {TEAM_TABLE_NAME} WHERE team_admin_id = $3 AND deactivated_at IS NULL) ) AND accepted_at IS NULL AND cancelled_at IS NULL"
        );

        let query_result = query(&query_body)
            .bind(&get_current_datetime())
            .bind(&user_email)
            .bind(&user_id)
            .execute(&mut **tx)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e).map_err(|e| e.into()),
        }
    }
}
