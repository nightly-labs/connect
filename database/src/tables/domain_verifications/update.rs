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
            "INSERT INTO {DOMAIN_VERIFICATIONS_TABLE_NAME} ({DOMAIN_VERIFICATIONS_KEYS}) VALUES ($1, $2, $3, $4, NULL, NULL, NULL)"
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
            "UPDATE {DOMAIN_VERIFICATIONS_TABLE_NAME} SET finished_at = $1 WHERE domain_name = $2 AND app_id = $3 AND finished_at IS NULL AND cancelled_at IS NULL AND deleted_at IS NULL"
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

    pub async fn cancel_domain_verification(
        &self,
        domain_name: &String,
        app_id: &String,
    ) -> Result<(), DbError> {
        let query_body = format!(
            "UPDATE {DOMAIN_VERIFICATIONS_TABLE_NAME} SET cancelled_at = $1 WHERE domain_name = $2 AND app_id = $3 AND finished_at IS NULL AND cancelled_at IS NULL AND deleted_at IS NULL"
        );

        let query_result = query(&query_body)
            .bind(&get_current_datetime())
            .bind(&domain_name)
            .bind(&app_id)
            .execute(&self.connection_pool)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e).map_err(|e| e.into()),
        }
    }

    // Sets finished domain verification as deleted aka inactive, this way the same domain can be verified again by the same or another app
    pub async fn delete_domain_verification(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        domain_name: &String,
        app_id: &String,
    ) -> Result<(), DbError> {
        let query_body = format!(
            "UPDATE {DOMAIN_VERIFICATIONS_TABLE_NAME} SET deleted_at = $1 WHERE domain_name = $2 AND app_id = $3 AND deleted_at IS NULL AND finished_at IS NOT NULL AND cancelled_at IS NULL"
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

    pub async fn delete_domain_verification_for_inactive_app(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        app_id: &String,
    ) -> Result<(), DbError> {
        let query_body = format!(
            "UPDATE {DOMAIN_VERIFICATIONS_TABLE_NAME} SET deleted_at = $1 WHERE app_id = $2 AND deleted_at IS NULL"
        );

        let query_result = query(&query_body)
            .bind(&get_current_datetime())
            .bind(&app_id)
            .execute(&mut **tx)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e).map_err(|e| e.into()),
        }
    }
    pub async fn delete_domain_verifications_for_inactive_apps(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        app_ids: &Vec<String>,
    ) -> Result<(), DbError> {
        if app_ids.is_empty() {
            return Ok(());
        }

        let query_body = format!(
            "UPDATE {DOMAIN_VERIFICATIONS_TABLE_NAME} 
             SET deleted_at = $1 
             WHERE app_id = ANY($2) 
             AND deleted_at IS NULL"
        );

        let query_result = sqlx::query(&query_body)
            .bind(&get_current_datetime())
            .bind(&app_ids)
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

    #[tokio::test]
    async fn test_domain_verifications() {
        let db = super::Db::connect_to_the_pool().await;
        db.truncate_all_tables().await.unwrap();

        let first_app_id = "first_app_id".to_string();
        let second_app_id = "second_app_id".to_string();

        let code = "code".to_string();

        // Start verification by first app for random domain
        db.create_new_domain_verification_entry(
            &"valid_domain_name_1".to_string(),
            &first_app_id,
            &code,
        )
        .await
        .unwrap();

        // Start verification by the second app for random domain
        db.create_new_domain_verification_entry(
            &"valid_domain_name_2".to_string(),
            &second_app_id,
            &code,
        )
        .await
        .unwrap();

        // Start verification by the second app for random domain
        db.create_new_domain_verification_entry(
            &"valid_domain_name_3".to_string(),
            &second_app_id,
            &code,
        )
        .await
        .unwrap();

        let test = db
            .get_pending_domain_verifications_by_app_ids(&vec![
                first_app_id.clone(),
                second_app_id.clone(),
            ])
            .await
            .unwrap();

        assert_eq!(test.len(), 2);
        assert_eq!(test.get(&first_app_id).unwrap().len(), 1);
        assert_eq!(test.get(&second_app_id).unwrap().len(), 2);

        // Cancel verification by the second app
        db.cancel_domain_verification(&"valid_domain_name_2".to_string(), &second_app_id)
            .await
            .unwrap();

        // Check
        let data = db
            .get_domain_verifications_by_app_id(&second_app_id)
            .await
            .unwrap();

        assert_eq!(data.len(), 2);
        // Find the cancelled verification
        let cancelled_verification = data
            .iter()
            .find(|v| v.domain_name == "valid_domain_name_2".to_string())
            .unwrap();

        assert!(cancelled_verification.cancelled_at.is_some());
    }

    #[tokio::test]
    async fn test_domain_verification_deletion() {
        let db = super::Db::connect_to_the_pool().await;
        db.truncate_all_tables().await.unwrap();

        let domain_name = "valid_domain_name".to_string();
        let app_id = "app_id".to_string();

        let code = "code".to_string();

        // Start verification by first app
        db.create_new_domain_verification_entry(&domain_name, &app_id, &code)
            .await
            .unwrap();

        let mut tx = db.connection_pool.begin().await.unwrap();

        // Delete verification
        db.delete_domain_verification(&mut tx, &domain_name, &app_id)
            .await
            .unwrap();

        tx.commit().await.unwrap();

        // Check
        let data = db
            .get_domain_verifications_by_app_id(&app_id)
            .await
            .unwrap();

        assert_eq!(data.len(), 1);
        assert!(data.get(0).unwrap().deleted_at.is_none());

        // FInish verification
        let mut tx = db.connection_pool.begin().await.unwrap();
        db.finish_domain_verification(&mut tx, &domain_name, &app_id)
            .await
            .unwrap();
        tx.commit().await.unwrap();

        // Check
        let data = db
            .get_domain_verifications_by_app_id(&app_id)
            .await
            .unwrap();

        assert_eq!(data.len(), 1);
        assert!(data.get(0).unwrap().finished_at.is_some());

        // Delete verification
        let mut tx = db.connection_pool.begin().await.unwrap();
        db.delete_domain_verification(&mut tx, &domain_name, &app_id)
            .await
            .unwrap();

        tx.commit().await.unwrap();

        // Check
        let data = db
            .get_domain_verifications_by_app_id(&app_id)
            .await
            .unwrap();

        assert_eq!(data.len(), 1);
        assert!(data.get(0).unwrap().deleted_at.is_some());
    }
}
