use super::table_struct::{DbRegisteredApp, REGISTERED_APPS_KEYS, REGISTERED_APPS_TABLE_NAME};
use crate::{db::Db, structs::db_error::DbError};
use sqlx::{query, Transaction};

impl Db {
    pub async fn register_new_app(&self, app: &DbRegisteredApp) -> Result<(), DbError> {
        let query_body = format!(
            "INSERT INTO {REGISTERED_APPS_TABLE_NAME} ({REGISTERED_APPS_KEYS}) VALUES ($1, $2, $3, $4, $5, $6)"
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
            "INSERT INTO {REGISTERED_APPS_TABLE_NAME} ({}) VALUES ($1, $2, $3, $4, $5, $6)",
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
        app_id: &str,
        domain: &str,
    ) -> Result<(), DbError> {
        let query_body = format!(
            "UPDATE {REGISTERED_APPS_TABLE_NAME} SET whitelisted_domains = array_append(whitelisted_domains, $1) WHERE app_id = $2",
        );

        let query_result = query(&query_body)
            .bind(domain)
            .bind(app_id)
            .execute(&self.connection_pool)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e).map_err(|e| e.into()),
        }
    }

    pub async fn remove_whitelisted_domain(
        &self,
        app_id: &str,
        domain: &str,
    ) -> Result<(), DbError> {
        let query_body = format!(
            "UPDATE {REGISTERED_APPS_TABLE_NAME} SET whitelisted_domains = array_remove(whitelisted_domains, $1) WHERE app_id = $2",
        );

        let query_result = query(&query_body)
            .bind(domain)
            .bind(app_id)
            .execute(&self.connection_pool)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e).map_err(|e| e.into()),
        }
    }
}
