use super::table_struct::{DbRegisteredApp, REGISTERED_APPS_KEYS, REGISTERED_APPS_TABLE_NAME};

use crate::{
    db::Db,
    structs::db_error::DbError,
    tables::{team::table_struct::TEAM_TABLE_NAME, utils::get_current_datetime},
};
use sqlx::{query, Transaction};

impl Db {
    pub async fn register_new_app(&self, app: &DbRegisteredApp) -> Result<(), DbError> {
        let query_body = format!(
            "INSERT INTO {REGISTERED_APPS_TABLE_NAME} ({REGISTERED_APPS_KEYS}) VALUES ($1, $2, $3, $4, $5, $6, NULL)",
        );

        let query_result = query(&query_body)
            .bind(&app.team_id)
            .bind(&app.app_id)
            .bind(&app.app_name)
            .bind(&app.whitelisted_domains)
            .bind(&app.ack_public_keys)
            .bind(&app.registration_timestamp)
            .execute(&self.connection_pool)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e).map_err(|e| e.into()),
        }
    }

    pub async fn register_new_app_within_tx(
        &self,
        tx: &mut Transaction<'_, sqlx::Postgres>,
        app: &DbRegisteredApp,
    ) -> Result<(), DbError> {
        let query_body = format!(
            "INSERT INTO {REGISTERED_APPS_TABLE_NAME} ({}) VALUES ($1, $2, $3, $4, $5, $6, NULL)",
            REGISTERED_APPS_KEYS
        );

        let query_result = query(&query_body)
            .bind(&app.team_id)
            .bind(&app.app_id)
            .bind(&app.app_name)
            .bind(&app.whitelisted_domains)
            .bind(&app.ack_public_keys)
            .bind(&app.registration_timestamp)
            .execute(&mut **tx)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e).map_err(|e| e.into()),
        }
    }

    pub async fn add_new_whitelisted_domain(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        app_id: &str,
        domain: &str,
    ) -> Result<(), DbError> {
        let query_body = format!(
            "UPDATE {REGISTERED_APPS_TABLE_NAME} SET whitelisted_domains = array_append(whitelisted_domains, $1) WHERE app_id = $2 AND deactivated_at IS NULL",
        );

        let query_result = query(&query_body)
            .bind(domain)
            .bind(app_id)
            .execute(&mut **tx)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e).map_err(|e| e.into()),
        }
    }

    pub async fn remove_whitelisted_domain(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        app_id: &str,
        domain: &str,
    ) -> Result<(), DbError> {
        let query_body = format!(
            "UPDATE {REGISTERED_APPS_TABLE_NAME} SET whitelisted_domains = array_remove(whitelisted_domains, $1) WHERE app_id = $2 AND deactivated_at IS NULL",
        );

        let query_result = query(&query_body)
            .bind(domain)
            .bind(app_id)
            .execute(&mut **tx)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e).map_err(|e| e.into()),
        }
    }

    pub async fn deactivate_app(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        app_id: &str,
    ) -> Result<(), DbError> {
        let query_body = format!(
            "UPDATE {REGISTERED_APPS_TABLE_NAME} SET deactivated_at = $1 WHERE app_id = $2 AND deactivated_at IS NULL",
        );

        let query_result = query(&query_body)
            .bind(&get_current_datetime())
            .bind(app_id)
            .execute(&mut **tx)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e).map_err(|e| e.into()),
        }
    }

    pub async fn deactivate_user_apps(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        user_id: &str,
    ) -> Result<(), DbError> {
        let query_body = format!(
            "UPDATE {REGISTERED_APPS_TABLE_NAME} SET deactivated_at = $1 WHERE team_id IN (SELECT team_id FROM {TEAM_TABLE_NAME} WHERE team_admin_id = $2 AND deactivated_at IS NULL) AND deactivated_at IS NULL",
        );

        let query_result = query(&query_body)
            .bind(&get_current_datetime())
            .bind(user_id)
            .execute(&mut **tx)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e).map_err(|e| e.into()),
        }
    }
}
