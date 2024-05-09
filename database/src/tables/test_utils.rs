#[cfg(feature = "cloud_integration_tests")]
#[cfg(test)]
pub mod test_utils {
    use crate::{
        db::Db,
        structs::{db_error::DbError, privilege_level::PrivilegeLevel},
        tables::{
            registered_app::table_struct::DbRegisteredApp, team::table_struct::Team,
            user_app_privileges::table_struct::UserAppPrivilege,
        },
    };
    use sqlx::{
        types::chrono::{DateTime, Utc},
        Row, Transaction,
    };

    impl Db {
        pub async fn truncate_all_tables(&self) -> Result<(), DbError> {
            let rows = sqlx::query(
                "SELECT table_name FROM information_schema.tables WHERE table_schema = 'public'",
            )
            .fetch_all(&self.connection_pool)
            .await?;

            if rows.is_empty() {
                println!("No tables to truncate");
                return Ok(());
            }

            let filter_list = vec![
                "_sqlx_migrations".to_string(),
                "spatial_ref_sys".to_string(),
                "geography_columns".to_string(),
                "geometry_columns".to_string(),
            ];

            // Join all names except _sqlx_migrations into a single string and run single truncate
            let tables_names = rows
                .iter()
                .map(|row| row.get::<String, &str>("table_name"))
                .filter(|table_name| !filter_list.contains(table_name))
                .collect::<Vec<String>>()
                .join(", ");

            let query = format!("TRUNCATE TABLE {tables_names} CASCADE");
            sqlx::query(&query).execute(&self.connection_pool).await?;

            // Reset all sequences
            let sequences = sqlx::query(
                "SELECT sequence_name FROM information_schema.sequences WHERE sequence_schema = 'public'",
            )
            .fetch_all(&self.connection_pool)
            .await?;

            for sequence in sequences {
                let seq_name = sequence.get::<String, &str>("sequence_name");
                let seq_reset_query = format!("ALTER SEQUENCE {} RESTART", seq_name);
                sqlx::query(&seq_reset_query)
                    .execute(&self.connection_pool)
                    .await?;
            }

            Ok(())
        }

        pub async fn refresh_continuous_aggregates(
            &self,
            views: Vec<String>,
        ) -> Result<(), DbError> {
            // Refresh views
            for view in views.iter() {
                let _ = sqlx::query(&format!(
                    "CALL refresh_continuous_aggregate('{view}', NULL, NULL)",
                    view = view
                ))
                .execute(&self.connection_pool)
                .await?;
            }

            println!("Refreshed {} continuous aggregates", views.len());

            Ok(())
        }

        pub async fn setup_test_team(
            &self,
            team_id: &String,
            app_id: &String,
            registration_timestamp: DateTime<Utc>,
        ) -> Result<(), DbError> {
            let user_id = "test_admin".to_string();
            let email = "admin".to_string();
            let password_hash = "pass_hash".to_string();

            self.add_new_user(&user_id, &email, Some(&password_hash), None)
                .await?;

            let team = Team {
                team_id: team_id.clone(),
                team_name: "test_team_name".to_string(),
                personal: false,
                subscription: None,
                team_admin_id: user_id.clone(),
                registration_timestamp: registration_timestamp,
            };

            let registered_app = DbRegisteredApp {
                team_id: team_id.clone(),
                app_id: app_id.clone(),
                app_name: format!("{app_id}_APP_NAME").to_string(),
                whitelisted_domains: vec!["localhost".to_string()],
                ack_public_keys: vec!["key".to_string()],
                registration_timestamp: registration_timestamp,
            };

            let admin_privilege = UserAppPrivilege {
                app_id: app_id.clone(),
                creation_timestamp: registration_timestamp,
                privilege_level: PrivilegeLevel::Admin,
                user_id: user_id.clone(),
            };

            // Start a transaction
            let mut tx: Transaction<'_, sqlx::Postgres> = self.connection_pool.begin().await?;

            // Attempt to create the new team within the transaction
            let create_team_result = self.create_new_team_within_tx(&mut tx, &team).await;
            if create_team_result.is_err() {
                // If creating the team fails, roll back the transaction and return the error
                tx.rollback().await?;
                return create_team_result.map_err(|err| err.into());
            }

            // Attempt to register the new app within the same transaction
            let register_app_result = self
                .register_new_app_within_tx(&mut tx, &registered_app)
                .await;
            if register_app_result.is_err() {
                // If registering the app fails, roll back the transaction and return the error
                tx.rollback().await?;
                return register_app_result.map_err(|err| err.into());
            }

            // Attempt to add team admin within the same transaction
            let add_admin_result = self
                .add_new_privilege_within_tx(&mut tx, &admin_privilege)
                .await;
            if add_admin_result.is_err() {
                // If adding the admin fails, roll back the transaction and return the error
                tx.rollback().await?;
                return add_admin_result.map_err(|err| err.into());
            }

            // If both actions succeeded, commit the transaction
            tx.commit().await?;
            Ok(())
        }
    }
}
