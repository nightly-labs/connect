#[cfg(test)]
pub mod test_utils {
    use crate::db::Db;
    use sqlx::Row;

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

        pub async fn refresh_continuous_aggregates(&self) -> Result<(), sqlx::Error> {
            // Refresh the hourly_requests_per_app view
            let _ = sqlx::query(
                "CALL refresh_continuous_aggregate('hourly_requests_per_app', NULL, NULL)",
            )
            .execute(&self.connection_pool)
            .await?;

            // Refresh the daily_requests_per_app view
            let _ = sqlx::query(
                "CALL refresh_continuous_aggregate('daily_requests_per_app', NULL, NULL)",
            )
            .execute(&self.connection_pool)
            .await?;

            Ok(())
        }
    }
}
