use crate::db::Db;
use sqlx::Postgres;
use sqlx::Transaction;

impl Db {
    pub async fn log_client_profile_merge_event(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        source_client_profile_id: i64,
        current_target_client_profile_id: Option<i64>,
        new_target_client_profile_id: i64,
    ) -> Result<(), sqlx::Error> {
        let query_body = format!(
            "INSERT INTO client_profile_merge_events (source_client_profile_id, current_target_client_profile_id new_target_client_profile_id) VALUES ($1, $2, $3)"
        );

        let query_result = sqlx::query(&query_body)
            .bind(source_client_profile_id)
            .bind(current_target_client_profile_id)
            .bind(new_target_client_profile_id)
            .execute(&mut **tx)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}
