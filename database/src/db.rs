use sqlx::{migrate, pool::PoolOptions, PgPool};

pub struct Db {
    pub connection_pool: PgPool,
}

impl Db {
    pub async fn connect_to_the_pool() -> Db {
        dotenvy::from_filename("infra/.env").unwrap();
        let db_name = std::env::var("POSTGRES_DB").expect("POSTGRES_DB must be set");
        let db_user = std::env::var("POSTGRES_USER").expect("POSTGRES_USER must be set");
        let db_password =
            std::env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");

        let pool = PoolOptions::new()
            .max_connections(50)
            .connect(
                format!(
                    "postgres://{}:{}@localhost:5432/{}",
                    db_user, db_password, db_name
                )
                .as_str(),
            )
            .await
            .unwrap();

        Db {
            connection_pool: pool,
        }
    }

    pub async fn migrate_tables(&self) -> Result<(), sqlx::migrate::MigrateError> {
        migrate!("./migrations").run(&self.connection_pool).await
    }

    pub async fn truncate_table(&self, table_name: &str) -> Result<(), sqlx::Error> {
        let query = format!("TRUNCATE TABLE {table_name}");
        sqlx::query(&query).execute(&self.connection_pool).await?;
        Ok(())
    }
}
