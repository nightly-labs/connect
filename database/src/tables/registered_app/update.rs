use super::table_struct::{RegisteredApp, REGISTERED_APPS_KEYS, REGISTERED_APPS_TABLE_NAME};
use crate::db::Db;
use sqlx::{query, Transaction};

impl Db {
    pub async fn register_new_app(&self, app: &RegisteredApp) -> Result<(), sqlx::Error> {
        let query_body = format!(
            "INSERT INTO {REGISTERED_APPS_TABLE_NAME} ({REGISTERED_APPS_KEYS}) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
        );

        let query_result = query(&query_body)
            .bind(&app.team_id)
            .bind(&app.app_id)
            .bind(&app.app_name)
            .bind(&app.whitelisted_domains)
            .bind(&app.ack_public_keys)
            .bind(&app.email)
            .bind(&(app.registration_timestamp as i64))
            .bind(&app.pass_hash)
            .execute(&self.connection_pool)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn register_new_app_within_tx(
        &self,
        tx: &mut Transaction<'_, sqlx::Postgres>,
        app: &RegisteredApp,
    ) -> Result<(), sqlx::Error> {
        let query_body = format!(
            "INSERT INTO {REGISTERED_APPS_TABLE_NAME} ({}) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
            REGISTERED_APPS_KEYS
        );

        let query_result = query(&query_body)
            .bind(&app.team_id)
            .bind(&app.app_id)
            .bind(&app.app_name)
            .bind(&app.whitelisted_domains)
            .bind(&app.ack_public_keys)
            .bind(&app.email)
            .bind(&(app.registration_timestamp as i64))
            .bind(&app.pass_hash)
            .execute(&mut **tx)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
