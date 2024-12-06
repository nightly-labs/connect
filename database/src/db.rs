use sqlx::{migrate, pool::PoolOptions, PgPool};

pub struct Db {
    pub connection_pool: PgPool,
}

impl Db {
    pub async fn connect_to_the_pool() -> Db {
        dotenvy::from_filename("infra/.env").unwrap();

        let db_address = std::env::var("DATABASE_ADDRESS").expect("POSTGRES_ADDRESS must be set");
        let db_port = std::env::var("DATABASE_PORT").expect("POSTGRES_PORT must be set");

        let db_name = std::env::var("POSTGRES_DB").expect("POSTGRES_DB must be set");
        let db_user = std::env::var("POSTGRES_USER").expect("POSTGRES_USER must be set");
        let db_password =
            std::env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");

        let connection_string = format!(
            "postgres://{}:{}@{}:{}/{}",
            db_user, db_password, db_address, db_port, db_name
        );

        println!("Connecting to the database... {:?}", connection_string);

        let pool = PoolOptions::new()
            .max_connections(50)
            .connect(connection_string.as_str())
            .await
            .unwrap();

        Db {
            connection_pool: pool,
        }
    }

    pub async fn migrate_tables(&self) -> Result<(), sqlx::migrate::MigrateError> {
        migrate!("./migrations").run(&self.connection_pool).await
    }
}
