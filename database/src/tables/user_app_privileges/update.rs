use std::collections::HashSet;

use super::table_struct::UserAppPrivilege;
use crate::db::Db;
use crate::structs::db_error::DbError;
use crate::structs::privilege_level::PrivilegeLevel;
use crate::tables::registered_app::table_struct::REGISTERED_APPS_TABLE_NAME;
use crate::tables::user_app_privileges::table_struct::{
    USER_APP_PRIVILEGES_KEYS, USER_APP_PRIVILEGES_TABLE_NAME,
};
use crate::tables::utils::get_current_datetime;
use sqlx::query;
use sqlx::Transaction;

impl Db {
    pub async fn add_new_privilege_within_tx(
        &self,
        tx: &mut Transaction<'_, sqlx::Postgres>,
        privilege: &UserAppPrivilege,
    ) -> Result<(), DbError> {
        let query_body = format!(
            "INSERT INTO {USER_APP_PRIVILEGES_TABLE_NAME} ({USER_APP_PRIVILEGES_KEYS}) VALUES ($1, $2, $3, $4)"
        );

        let query_result = query(&query_body)
            .bind(&privilege.user_id)
            .bind(&privilege.app_id)
            .bind(&privilege.privilege_level)
            .bind(&privilege.creation_timestamp)
            .execute(&mut **tx)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e).map_err(|e| e.into()),
        }
    }

    pub async fn add_new_privilege(&self, privilege: &UserAppPrivilege) -> Result<(), DbError> {
        let query_body = format!(
            "INSERT INTO {USER_APP_PRIVILEGES_TABLE_NAME} ({USER_APP_PRIVILEGES_KEYS}) VALUES ($1, $2, $3, $4)"
        );

        let query_result = query(&query_body)
            .bind(&privilege.user_id)
            .bind(&privilege.app_id)
            .bind(&privilege.privilege_level)
            .bind(&privilege.creation_timestamp)
            .execute(&self.connection_pool)
            .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e).map_err(|e| e.into()),
        }
    }

    pub async fn add_user_to_the_team(
        &self,
        user_id: &String,
        team_id: &String,
    ) -> Result<(), DbError> {
        // Retrieve all apps associated with the team
        let apps_query =
            format!("SELECT app_id FROM {REGISTERED_APPS_TABLE_NAME} WHERE team_id = $1");
        let apps: Vec<String> = sqlx::query_as(&apps_query)
            .bind(team_id)
            .fetch_all(&self.connection_pool)
            .await?
            .into_iter()
            .map(|(app_id,): (String,)| app_id)
            .collect();

        // Only proceed if there are apps to assign privileges for
        if apps.is_empty() {
            return Err(DbError::DatabaseError(
                "No apps associated with the team".to_string(),
            ));
        }

        // Build values list for insertion
        let values_list: Vec<String> = apps
            .iter()
            .map(|app_id| {
                format!(
                    "('{}', '{}', '{:?}', '{}')",
                    user_id,
                    app_id.clone(),
                    PrivilegeLevel::Read,
                    get_current_datetime()
                )
            })
            .collect();

        let values_str = values_list.join(", ");
        let insert_query = format!(
            "INSERT INTO {USER_APP_PRIVILEGES_TABLE_NAME} ({USER_APP_PRIVILEGES_KEYS}) VALUES {}",
            values_str
        );

        sqlx::query(&insert_query)
            .execute(&self.connection_pool)
            .await?;

        Ok(())
    }

    pub async fn remove_user_from_the_team(
        &self,
        user_id: &String,
        team_id: &String,
    ) -> Result<(), DbError> {
        // Retrieve all apps associated with the team
        let apps_query =
            format!("SELECT app_id FROM {REGISTERED_APPS_TABLE_NAME} WHERE team_id = $1");
        let apps: Vec<String> = sqlx::query_as(&apps_query)
            .bind(team_id)
            .fetch_all(&self.connection_pool)
            .await?
            .into_iter()
            .map(|(app_id,): (String,)| app_id)
            .collect();

        // Only proceed if there are apps to assign privileges for
        if apps.is_empty() {
            return Err(DbError::DatabaseError(
                "No apps associated with the team".to_string(),
            ));
        }

        // Start a transaction
        let mut tx = self.connection_pool.begin().await?;

        for app_id in apps.iter() {
            let delete_query = format!(
                "DELETE FROM {USER_APP_PRIVILEGES_TABLE_NAME} WHERE user_id = $1 AND app_id = $2",
            );

            // If any of the queries fail, rollback the transaction
            if let Err(err) = sqlx::query(&delete_query)
                .bind(user_id)
                .bind(app_id)
                .execute(&mut *tx)
                .await
            {
                let _ = tx.rollback().await;
                return Err(err.into());
            }
        }

        // Commit the transaction
        tx.commit().await?;

        Ok(())
    }

    pub async fn add_privileges_for_new_team_app_for_existing_users(
        &self,
        tx: &mut Transaction<'_, sqlx::Postgres>,
        team_id: &String,
        team_admin_id: &String,
        app_id: &String,
    ) -> Result<(), DbError> {
        // Get all users that are part of the team
        let users_privileges_query = self.get_privileges_by_team_id(team_id).await?;
        let mut users_ids_list = users_privileges_query
            .into_iter()
            .map(|privilege| privilege.user_id)
            .collect::<HashSet<String>>();

        // Remove team admin from the list
        users_ids_list.remove(team_admin_id);

        // If list is empty, there is no need to proceed
        if users_ids_list.is_empty() {
            return Ok(());
        }

        // Build values list for insertion
        let values_list: Vec<String> = users_ids_list
            .iter()
            .map(|user_id| {
                format!(
                    "('{}', '{}', '{:?}', '{}')",
                    user_id,
                    app_id.clone(),
                    PrivilegeLevel::Read,
                    get_current_datetime()
                )
            })
            .collect();

        let values_str = values_list.join(", ");
        let insert_query = format!(
            "INSERT INTO {USER_APP_PRIVILEGES_TABLE_NAME} ({USER_APP_PRIVILEGES_KEYS}) VALUES {}",
            values_str
        );

        sqlx::query(&insert_query).execute(&mut **tx).await?;

        Ok(())
    }
}

#[cfg(feature = "cloud_db_tests")]
#[cfg(test)]
mod tests {
    use crate::{
        structs::privilege_level::PrivilegeLevel,
        tables::{
            grafana_users::table_struct::GrafanaUser,
            registered_app::table_struct::DbRegisteredApp, team::table_struct::Team,
            user_app_privileges::table_struct::UserAppPrivilege, utils::to_microsecond_precision,
        },
    };
    use sqlx::types::chrono::Utc;

    #[tokio::test]
    async fn test_add_new_privilege() {
        let db = super::Db::connect_to_the_pool().await;
        db.truncate_all_tables().await.unwrap();

        // Create test team instance
        let team_id = "test_team_id".to_string();
        let app_id = "test_app_id".to_string();

        db.setup_test_team(&team_id, &app_id, Utc::now())
            .await
            .unwrap();

        let user = GrafanaUser {
            email: "test_user_email".to_string(),
            password_hash: "test_password_hash".to_string(),
            user_id: "test_user_id".to_string(),
            creation_timestamp: to_microsecond_precision(&Utc::now()),
        };
        db.add_new_user(&user).await.unwrap();

        let privilege = UserAppPrivilege {
            user_id: user.user_id.clone(),
            app_id: app_id.clone(),
            privilege_level: PrivilegeLevel::Edit,
            creation_timestamp: to_microsecond_precision(&Utc::now()),
        };
        db.add_new_privilege(&privilege).await.unwrap();

        let get_by_user_id_and_app_id = db
            .get_privilege_by_user_id_and_app_id(&user.user_id, &app_id)
            .await
            .unwrap();
        assert_eq!(privilege, get_by_user_id_and_app_id);

        let get_by_user_id = db.get_privileges_by_user_id(&user.user_id).await.unwrap();
        assert_eq!(vec![privilege.clone()], get_by_user_id);

        let get_by_app_id = db.get_privileges_by_app_id(&app_id).await.unwrap();
        assert!(get_by_app_id.len() == 2);
        assert!(get_by_app_id.contains(&privilege));
    }

    #[tokio::test]
    async fn test_add_user_to_team() {
        let db = super::Db::connect_to_the_pool().await;
        db.truncate_all_tables().await.unwrap();

        // Create test team instance
        let team_id = "test_team_id".to_string();
        let app_id = "test_app_id".to_string();

        db.setup_test_team(&team_id, &app_id, Utc::now())
            .await
            .unwrap();

        let user = GrafanaUser {
            email: "test_user_email".to_string(),
            password_hash: "test_password_hash".to_string(),
            user_id: "test_user_id".to_string(),
            creation_timestamp: to_microsecond_precision(&Utc::now()),
        };
        db.add_new_user(&user).await.unwrap();

        // Create 7 more apps under the same team
        for i in 0..7 {
            let app_id = format!("test_app_id_{}", i);
            let app = DbRegisteredApp {
                ack_public_keys: vec!["test_ack_public_key".to_string()],
                whitelisted_domains: vec!["test_whitelisted_domain".to_string()],
                app_id: app_id.clone(),
                app_name: format!("test_app_name_{}", i),
                team_id: team_id.clone(),
                registration_timestamp: to_microsecond_precision(&Utc::now()),
            };

            db.register_new_app(&app).await.unwrap();
        }

        db.add_user_to_the_team(&user.user_id, &team_id)
            .await
            .unwrap();

        let get_by_user_id = db.get_privileges_by_user_id(&user.user_id).await.unwrap();
        assert!(get_by_user_id.len() == 8);

        // Create new team
        let team_id = "test_team_id_2".to_string();

        let team = Team {
            team_id: team_id.clone(),
            team_name: "test_team_name".to_string(),
            personal: false,
            subscription: None,
            team_admin_id: "test_team_admin_id".to_string(),
            registration_timestamp: to_microsecond_precision(&Utc::now()),
        };

        db.create_new_team(&team).await.unwrap();

        // Add 2 apps to the new team
        for i in 0..2 {
            let app_id = format!("test_app_id_2_{}", i);
            let app = DbRegisteredApp {
                ack_public_keys: vec!["test_ack_public_key".to_string()],
                whitelisted_domains: vec!["test_whitelisted_domain".to_string()],
                app_id: app_id.clone(),
                app_name: format!("test_app_name_{}", i),
                team_id: team_id.clone(),
                registration_timestamp: to_microsecond_precision(&Utc::now()),
            };

            db.register_new_app(&app).await.unwrap();
        }

        // Add user to the new team
        db.add_user_to_the_team(&user.user_id, &team_id)
            .await
            .unwrap();

        let get_by_user_id = db.get_privileges_by_user_id(&user.user_id).await.unwrap();
        assert!(get_by_user_id.len() == 10);
    }
}
