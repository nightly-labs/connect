use crate::{
    db::Db,
    structs::{
        db_error::DbError, domain_verification_status::DomainVerificationStatus,
        whitelisted_domain::WhitelistedDomain,
    },
    tables::domain_verifications::table_struct::{
        DomainVerification, DOMAIN_VERIFICATIONS_TABLE_NAME,
    },
};
use sqlx::query_as;
use std::collections::HashMap;

impl Db {
    pub async fn get_domain_verifications_by_app_id(
        &self,
        app_id: &String,
    ) -> Result<Vec<DomainVerification>, DbError> {
        let query = format!("SELECT * FROM {DOMAIN_VERIFICATIONS_TABLE_NAME} WHERE app_id = $1 ORDER BY created_at DESC");
        let typed_query = query_as::<_, DomainVerification>(&query);

        return typed_query
            .bind(&app_id)
            .fetch_all(&self.connection_pool)
            .await
            .map_err(|e| e.into());
    }

    pub async fn get_domain_verification_by_domain_name(
        &self,
        domain_name: &String,
    ) -> Result<Option<DomainVerification>, DbError> {
        let query =
            format!("SELECT * FROM {DOMAIN_VERIFICATIONS_TABLE_NAME} WHERE domain_name = $1");
        let typed_query = query_as::<_, DomainVerification>(&query);

        return typed_query
            .bind(&domain_name)
            .fetch_optional(&self.connection_pool)
            .await
            .map_err(|e| e.into());
    }

    pub async fn get_domain_verification_by_domain_name_and_app_id(
        &self,
        domain_name: &String,
        app_id: &String,
    ) -> Result<Option<DomainVerification>, DbError> {
        let query =
            format!("SELECT * FROM {DOMAIN_VERIFICATIONS_TABLE_NAME} WHERE domain_name = $1 AND app_id = $2");
        let typed_query = query_as::<_, DomainVerification>(&query);

        return typed_query
            .bind(&domain_name)
            .bind(&app_id)
            .fetch_optional(&self.connection_pool)
            .await
            .map_err(|e| e.into());
    }

    pub async fn get_pending_domain_verifications_by_app_ids(
        &self,
        app_ids: &Vec<String>,
    ) -> Result<HashMap<String, Vec<WhitelistedDomain>>, DbError> {
        let query = format!(
            "SELECT app_id, domain_name FROM {DOMAIN_VERIFICATIONS_TABLE_NAME} WHERE app_id = ANY($1) AND finished_at IS NULL AND cancelled_at IS NULL"
        );
        let typed_query = query_as::<_, (String, String)>(&query);

        let mut app_id_to_domain_names: HashMap<String, Vec<WhitelistedDomain>> = HashMap::new();

        let rows = typed_query
            .bind(app_ids)
            .fetch_all(&self.connection_pool)
            .await
            .map_err(|e| DbError::from(e))?;

        for row in rows {
            let (app_id, domain_name) = row;

            app_id_to_domain_names
                .entry(app_id)
                .or_insert_with(Vec::new)
                .push(WhitelistedDomain {
                    domain: domain_name,
                    status: DomainVerificationStatus::Pending,
                });
        }

        return Ok(app_id_to_domain_names);
    }
}
