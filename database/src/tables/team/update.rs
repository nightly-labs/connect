use super::table_struct::TEAM_KEYS;
use crate::{
    db::Db,
    structs::subscription::Subscription,
    tables::team::table_struct::{Team, TEAM_TABLE_NAME},
};
use sqlx::{query, Transaction};

impl Db {
    pub async fn create_new_team_within_tx(
        &self,
        tx: &mut Transaction<'_, sqlx::Postgres>,
        team: &Team,
    ) -> Result<(), sqlx::Error> {
        let query_body =
            format!("INSERT INTO {TEAM_TABLE_NAME} ({TEAM_KEYS}) VALUES ($1, $2, $3, $4, $5, $6)");

        let query_result = query(&query_body)
            .bind(&team.team_id)
            .bind(&team.team_name)
            .bind(&team.personal)
            .bind(&team.subscription)
            .bind(&team.team_admin_id)
            .bind(&team.registration_timestamp)
            .execute(&mut **tx)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn create_new_team(&self, team: &Team) -> Result<(), sqlx::Error> {
        let query_body =
            format!("INSERT INTO {TEAM_TABLE_NAME} ({TEAM_KEYS}) VALUES ($1, $2, $3, $4, $5, $6)");

        let query_result = query(&query_body)
            .bind(&team.team_id)
            .bind(&team.team_name)
            .bind(&team.personal)
            .bind(&team.subscription)
            .bind(&team.team_admin_id)
            .bind(&team.registration_timestamp)
            .execute(&self.connection_pool)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub async fn update_subscription(
        &self,
        team_id: &String,
        subscription: &Subscription,
    ) -> Result<(), sqlx::Error> {
        let query_body =
            format!("UPDATE {TEAM_TABLE_NAME} SET subscription = $1 WHERE team_id = $2");
        let query_result = query(&query_body)
            .bind(subscription)
            .bind(team_id)
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
    use crate::tables::{
        grafana_users::table_struct::GrafanaUser, team::table_struct::Team,
        utils::to_microsecond_precision,
    };
    use sqlx::types::chrono::Utc;

    #[tokio::test]
    async fn test_create_team() {
        let db = super::Db::connect_to_the_pool().await;
        db.truncate_all_tables().await.unwrap();

        // First create a user
        let admin = GrafanaUser {
            email: "test_email".to_string(),
            password_hash: "test_password_hash".to_string(),
            user_id: "test_user_id".to_string(),
            creation_timestamp: to_microsecond_precision(&Utc::now()),
        };

        db.add_new_user(&admin).await.unwrap();

        // Create team and register app
        let team = Team {
            team_id: "test_team_id".to_string(),
            team_name: "test_team_name".to_string(),
            personal: false,
            subscription: None,
            team_admin_id: "test_team_admin_id".to_string(),
            registration_timestamp: to_microsecond_precision(&Utc::now()),
        };

        db.create_new_team(&team).await.unwrap();

        let team_result = db.get_team_by_team_id(None, &team.team_id).await.unwrap();
        assert_eq!(team_result, Some(team));
    }
}
