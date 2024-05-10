use super::table_struct::{DOMAIN_VERIFICATIONS_KEYS, DOMAIN_VERIFICATIONS_TABLE_NAME};
use crate::db::Db;
use crate::structs::db_error::DbError;
use crate::tables::utils::get_current_datetime;
use sqlx::query;

impl Db {
    pub async fn create_new_domain_verification_entry(
        &self,
        domain_name: &String,
        app_id: &String,
        code: &String,
    ) -> Result<(), DbError> {
        let query_body = format!(
            "INSERT INTO {DOMAIN_VERIFICATIONS_TABLE_NAME} ({DOMAIN_VERIFICATIONS_KEYS}) VALUES ($1, $2, $3, $4, NULL)"
        );

        let query_result = query(&query_body)
            .bind(&domain_name)
            .bind(&app_id)
            .bind(&code)
            .bind(&get_current_datetime())
            .execute(&self.connection_pool)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e).map_err(|e| e.into()),
        }
    }

    pub async fn finish_domain_verification(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        domain_name: &String,
        app_id: &String,
    ) -> Result<(), DbError> {
        let query_body = format!(
            "UPDATE {DOMAIN_VERIFICATIONS_TABLE_NAME} SET finished_at = $1 WHERE domain_name = $2 AND app_id = $3 AND finished_at IS NULL"
        );

        let query_result = query(&query_body)
            .bind(&get_current_datetime())
            .bind(&domain_name)
            .bind(&app_id)
            .execute(&mut **tx)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e).map_err(|e| e.into()),
        }
    }
}

#[cfg(feature = "cloud_integration_tests")]
#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn test_domain_verification() {
        let db = super::Db::connect_to_the_pool().await;
        db.truncate_all_tables().await.unwrap();

        let domain_name = "valid_domain_name".to_string();
        let first_app_id = "first_app_id".to_string();
        let second_app_id = "second_app_id".to_string();

        let code = "code".to_string();

        // Start verification by first app
        db.create_new_domain_verification_entry(&domain_name, &first_app_id, &code)
            .await
            .unwrap();

        // Try to verify the same domain by the same app
        db.create_new_domain_verification_entry(&domain_name, &first_app_id, &code)
            .await
            .unwrap_err();

        // Try to start verification by the second app for the same domain
        db.create_new_domain_verification_entry(&domain_name, &second_app_id, &code)
            .await
            .unwrap();

        // Finish verification
        let mut tx = db.connection_pool.begin().await.unwrap();
        db.finish_domain_verification(&mut tx, &domain_name, &first_app_id)
            .await
            .unwrap();
        tx.commit().await.unwrap();

        // Try to finish verification of the same domain, should fail
        let mut tx = db.connection_pool.begin().await.unwrap();
        db.finish_domain_verification(&mut tx, &domain_name, &second_app_id)
            .await
            .unwrap_err();
        tx.rollback().await.unwrap();
    }
}
