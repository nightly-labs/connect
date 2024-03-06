#[cfg(test)]
pub mod test_utils {
    use crate::{
        db::Db,
        structs::privelage_level::PrivilegeLevel,
        tables::{
            grafana_users::table_struct::GrafanaUser,
            registered_app::table_struct::DbRegisteredApp, team::table_struct::Team,
            user_app_privileges::table_struct::UserAppPrivilege,
        },
    };
    use sqlx::{
        types::chrono::{DateTime, Utc},
        Row,
    };

    impl Db {
        pub async fn truncate_all_tables(&self) -> Result<(), sqlx::Error> {
            let rows = sqlx::query(
                "SELECT table_name FROM information_schema.tables WHERE table_schema = 'public'",
            )
            .fetch_all(&self.connection_pool)
            .await?;

            if rows.is_empty() {
                println!("No tables to truncate");
                return Ok(());
            }

            // Join all names except _sqlx_migrations into a single string and run single truncate
            let tables_names = rows
                .iter()
                .map(|row| row.get::<String, &str>("table_name"))
                .filter(|table_name| !table_name.starts_with("_sqlx_migrations"))
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
        ) -> Result<(), sqlx::Error> {
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
        ) -> anyhow::Result<()> {
            let admin = GrafanaUser {
                creation_timestamp: registration_timestamp,
                email: "email".to_string(),
                password_hash: "pass_hash".to_string(),
                user_id: "test_admin".to_string(),
            };

            self.add_new_user(&admin).await?;

            let team = Team {
                team_id: team_id.clone(),
                team_name: "test_team_name".to_string(),
                personal: false,
                subscription: None,
                team_admin_id: admin.user_id.clone(),
                registration_timestamp: registration_timestamp,
            };

            let registered_app = DbRegisteredApp {
                team_id: team_id.clone(),
                app_id: app_id.clone(),
                app_name: "test_app".to_string(),
                whitelisted_domains: vec!["localhost".to_string()],
                subscription: None,
                ack_public_keys: vec!["key".to_string()],
                email: None,
                pass_hash: None,
                registration_timestamp: registration_timestamp,
            };

            let admin_privilege = UserAppPrivilege {
                app_id: app_id.clone(),
                creation_timestamp: registration_timestamp,
                privilege_level: PrivilegeLevel::Admin,
                user_id: admin.user_id.clone(),
            };

            match self
                .setup_team(&team, &registered_app, &admin_privilege)
                .await
            {
                Ok(_) => Ok(()),
                Err(e) => Err(anyhow::anyhow!(e)),
            }
        }
    }
}
