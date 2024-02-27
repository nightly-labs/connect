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
    use crate::{
        structs::privelage_level::PrivilegeLevel,
        tables::{grafana_users::table_struct::GrafanaUser, utils::to_microsecond_precision},
    };
    use sqlx::types::chrono::Utc;

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

        let user = GrafanaUser {
            name: "test_user".to_string(),
            team_id: team_id.to_string(),
            team_admin: false,
            email: "test_user_email".to_string(),
            password_hash: "test_password_hash".to_string(),
            privilege_level: PrivilegeLevel::Read,
            creation_timestamp: to_microsecond_precision(&Utc::now()),
        };

        db.add_new_user(&user).await.unwrap();

        let user_result = db.get_user_by_user_name(&user.name).await.unwrap();
        assert_eq!(user_result, user);
    }
}
