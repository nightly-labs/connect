#[cfg(test)]
pub mod test_utils {
    use crate::{
        db::Db,
        tables::{registered_app::table_struct::RegisteredApp, team::table_struct::Team},
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
            let team = Team {
                team_id: team_id.clone(),
                subscription: None,
                team_admin_id: "admin".to_string(),
                registration_timestamp: registration_timestamp,
            };

            let registered_app = RegisteredApp {
                team_id: team_id.clone(),
                app_id: app_id.clone(),
                app_name: "test_app".to_string(),
                whitelisted_domains: vec!["localhost".to_string()],
                subscription: None,
                ack_public_keys: vec!["key".to_string()],
                email: None,
                pass_hash: None,
                registration_timestamp: registration_timestamp.timestamp() as u64,
            };

            match self
                .create_team_and_register_app(&team, &registered_app)
                .await
            {
                Ok(_) => Ok(()),
                Err(e) => Err(anyhow::anyhow!(e)),
            }
        }
    }
}
