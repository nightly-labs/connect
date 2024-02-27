use super::table_struct::{GrafanaUser, GRAFANA_USERS_KEYS, GRAFANA_USERS_TABLE_NAME};
use crate::db::Db;
use sqlx::query;
use sqlx::Transaction;

impl Db {
    pub async fn create_new_user_within_tx(
        &self,
        tx: &mut Transaction<'_, sqlx::Postgres>,
        user: &GrafanaUser,
    ) -> Result<(), sqlx::Error> {
        let query_body = format!(
            "INSERT INTO {GRAFANA_USERS_TABLE_NAME} ({GRAFANA_USERS_KEYS}) VALUES ($1, $2, $3, $4, $5, $6, $7)"
        );

        let query_result = query(&query_body)
            .bind(&user.name)
            .bind(&user.team_id)
            .bind(&user.team_admin)
            .bind(&user.email)
            .bind(&user.password_hash)
            .bind(&user.privilege_level)
            .bind(&user.creation_timestamp)
            .execute(&mut **tx)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn add_new_user(&self, user: &GrafanaUser) -> Result<(), sqlx::Error> {
        let query_body = format!(
            "INSERT INTO {GRAFANA_USERS_TABLE_NAME} ({GRAFANA_USERS_KEYS}) VALUES ($1, $2, $3, $4, $5, $6, $7)"
        );

        let query_result = query(&query_body)
            .bind(&user.name)
            .bind(&user.team_id)
            .bind(&user.email)
            .bind(&user.password_hash)
            .bind(&user.privilege_level)
            .bind(&user.creation_timestamp)
            .execute(&self.connection_pool)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tables::sessions::table_struct::DbNcSession;
    use sqlx::types::chrono::Utc;
    use std::time::Duration;

    #[tokio::test]
    async fn test_create_user() {
        let db = super::Db::connect_to_the_pool().await;
        db.truncate_all_tables().await.unwrap();

        // Create test team instance
        let team_id = "test_team_id".to_string();
        let app_id = "test_app_id".to_string();

        db.setup_test_team(&team_id, &app_id, Utc::now())
            .await
            .unwrap();

        let now = Utc::now();
        let start_of_first_period = now - Duration::from_secs(60 * 60 * 24 * 60); // Start of first period, 60 days ago
        let start_of_second_period = now - Duration::from_secs(60 * 60 * 24 * 30); // Start of second period, 30 days ago
        let num_sessions_first_period: u64 = 40;
        let num_sessions_second_period: u64 = 28;

        // Generate and save sessions for the first period
        for i in 0..num_sessions_first_period {
            let session_start = start_of_first_period
                + Duration::from_secs(i * 86400 / num_sessions_first_period as u64);
            let session_end = session_start + Duration::from_secs(60 * 30); // Duration of 30 minutes for each session

            let session = DbNcSession {
                session_id: format!("session_{}_{}", app_id, i),
                app_id: app_id.clone(),
                app_metadata: "test_metadata".to_string(),
                app_ip_address: "127.0.0.1".to_string(),
                persistent: false,
                network: "test_network".to_string(),
                client: None,
                session_open_timestamp: session_start,
                session_close_timestamp: Some(session_end),
            };

            db.save_new_session(&session).await.unwrap();
            db.close_session(&session.session_id, session_end)
                .await
                .unwrap();
        }

        // Generate and save sessions for the second period
        for i in 0..num_sessions_second_period {
            let session_start = start_of_second_period
                + Duration::from_secs(i * 86400 / num_sessions_second_period as u64);
            let session_end = session_start + Duration::from_secs(60 * 30); // Duration of 30 minutes for each session

            let session = DbNcSession {
                session_id: format!("session_{}_{}_2nd", app_id, i), // Ensure unique session IDs for the second period
                app_id: app_id.clone(),
                app_metadata: "test_metadata".to_string(),
                app_ip_address: "127.0.0.1".to_string(),
                persistent: false,
                network: "test_network".to_string(),
                client: None,
                session_open_timestamp: session_start,
                session_close_timestamp: Some(session_end),
            };

            db.save_new_session(&session).await.unwrap();
            db.close_session(&session.session_id, session_end)
                .await
                .unwrap();
        }

        // Manually refresh the continuous aggregates
        db.refresh_continuous_aggregates(vec![
            "sessions_stats_per_app_daily".to_string(),
            "sessions_stats_per_app_monthly".to_string(),
        ])
        .await
        .unwrap();

        let stats = db.get_monthly_sessions_stats(&app_id).await.unwrap();

        assert_eq!(stats.len(), 2);
    }
}
