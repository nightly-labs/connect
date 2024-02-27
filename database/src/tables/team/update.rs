use super::table_struct::TEAM_KEYS;
use crate::{
    db::Db,
    structs::subscription::Subscription,
    tables::{
        grafana_users::table_struct::GrafanaUser,
        registered_app::table_struct::RegisteredApp,
        team::table_struct::{Team, TEAM_TABLE_NAME},
    },
};
use sqlx::{query, Transaction};

impl Db {
    pub async fn create_new_team_within_tx(
        &self,
        tx: &mut Transaction<'_, sqlx::Postgres>,
        team: &Team,
    ) -> Result<(), sqlx::Error> {
        let query_body =
            format!("INSERT INTO {TEAM_TABLE_NAME} ({TEAM_KEYS}) VALUES ($1, $2, $3, $4)");

        let query_result = query(&query_body)
            .bind(&team.team_id)
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

    pub async fn setup_team(
        &self,
        team: &Team,
        app: &RegisteredApp,
        admin: &GrafanaUser,
    ) -> Result<(), sqlx::Error> {
        // Start a transaction
        let mut tx: Transaction<'_, sqlx::Postgres> = self.connection_pool.begin().await?;

        // Attempt to create the new team within the transaction
        let create_team_result = self.create_new_team_within_tx(&mut tx, team).await;
        if create_team_result.is_err() {
            // If creating the team fails, roll back the transaction and return the error
            tx.rollback().await?;
            return create_team_result;
        }

        // Attempt to add team admin within the same transaction
        let add_admin_result = self.create_new_user_within_tx(&mut tx, admin).await;
        if add_admin_result.is_err() {
            // If adding the admin fails, roll back the transaction and return the error
            tx.rollback().await?;
            return add_admin_result;
        }

        // Attempt to register the new app within the same transaction
        let register_app_result = self.register_new_app_within_tx(&mut tx, app).await;
        if register_app_result.is_err() {
            // If registering the app fails, roll back the transaction and return the error
            tx.rollback().await?;
            return register_app_result;
        }

        // If both actions succeeded, commit the transaction
        tx.commit().await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        structs::privelage_level::PrivilegeLevel,
        tables::{
            grafana_users::table_struct::GrafanaUser, registered_app::table_struct::RegisteredApp,
            team::table_struct::Team, utils::to_microsecond_precision,
        },
    };
    use sqlx::types::chrono::Utc;

    #[tokio::test]
    async fn test_create_team() {
        let db = super::Db::connect_to_the_pool().await;
        db.truncate_all_tables().await.unwrap();

        // Create team and register app
        let team = Team {
            team_id: "test_team_id".to_string(),
            subscription: None,
            team_admin_id: "test_team_admin_id".to_string(),
            registration_timestamp: to_microsecond_precision(&Utc::now()),
        };

        let app = RegisteredApp {
            app_id: "test_app_id".to_string(),
            team_id: "test_team_id".to_string(),
            app_name: "test_app_name".to_string(),
            ack_public_keys: vec!["test_ack_public_key".to_string()],
            whitelisted_domains: vec!["test_whitelisted_domain".to_string()],
            email: None,
            pass_hash: None,
            registration_timestamp: 0,
            subscription: None,
        };

        let admin = GrafanaUser {
            name: "test_team_admin_id".to_string(),
            team_id: "test_team_id".to_string(),
            email: "test_email".to_string(),
            password_hash: "test_password_hash".to_string(),
            privilege_level: PrivilegeLevel::Admin,
            creation_timestamp: to_microsecond_precision(&Utc::now()),
            team_admin: true,
        };

        db.setup_team(&team, &app, &admin).await.unwrap();

        let team_result = db.get_team_by_team_id(None, &team.team_id).await.unwrap();
        assert_eq!(team_result, team);

        let admin_result = db.get_user_by_user_name(&admin.name).await.unwrap();
        assert_eq!(admin_result, admin);

        let app_result = db.get_registered_app_by_app_id(&app.app_id).await.unwrap();
        assert_eq!(app_result, app);
    }
}
