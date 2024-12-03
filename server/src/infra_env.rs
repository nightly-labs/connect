#![allow(non_snake_case)]
use once_cell::sync::OnceCell;

#[derive(Debug)]
pub struct INFRA_ENV {
    pub ENVIRONMENT: String,
    pub ONLY_DATABASE: bool,
    pub DATABASE_ADDRESS: String,
    pub DATABASE_PORT: String,
    pub POSTGRES_USER: String,
    pub POSTGRES_PASSWORD: String,
    pub POSTGRES_DB: String,
    pub PG_DATA: String,
    pub GRAFANA_DB_USERNAME: String,
    pub GRAFANA_DB_PASSWORD: String,
    pub TIMESCALEDB_IMAGE: String,
    pub OFELIA_IMAGE: String,
    pub TIMESCALEDB_DATA: String,
    pub TIMESCALEDB_BACKUPS: String,
    pub TIMESCALEDB_LOGS: String,
    pub TIMESCALEDB_PGBACKREST_CONFIG: String,
    pub OFELIA_LOGS: String,
    pub MANUAL_BACKUPS: String,
    pub CUSTOM_ENTRYPOINT: String,
    pub OFELIA_SMTP_HOST: String,
    pub OFELIA_SMTP_PORT: String,
    pub OFELIA_SMTP_USER: String,
    pub OFELIA_SMTP_PASSWORD: String,
    pub OFELIA_EMAIL_FROM: String,
    pub OFELIA_EMAIL_TO: String,
}

pub fn get_env() -> &'static INFRA_ENV {
    static INSTANCE: OnceCell<INFRA_ENV> = OnceCell::new();

    INSTANCE.get_or_init(|| {
        dotenvy::from_filename(".env").ok();

        let env = INFRA_ENV {
            ENVIRONMENT: std::env::var("ENV").unwrap_or_else(|_| "DEV".to_string()), // Default to DEV if not set
            ONLY_DATABASE: std::env::var("ONLY_DATABASE")
                .unwrap_or_else(|_| "false".to_string())
                .eq_ignore_ascii_case("true"),
            DATABASE_ADDRESS: std::env::var("DATABASE_ADDRESS")
                .expect("Failed to get DATABASE_ADDRESS env"),
            DATABASE_PORT: std::env::var("DATABASE_PORT").unwrap_or_else(|_| "5432".to_string()), // Default to 5432 if not set
            POSTGRES_USER: std::env::var("POSTGRES_USER").expect("Failed to get POSTGRES_USER env"),
            POSTGRES_PASSWORD: std::env::var("POSTGRES_PASSWORD")
                .expect("Failed to get POSTGRES_PASSWORD env"),
            POSTGRES_DB: std::env::var("POSTGRES_DB").expect("Failed to get POSTGRES_DB env"),
            PG_DATA: std::env::var("PG_DATA").expect("Failed to get PG_DATA env"),
            GRAFANA_DB_USERNAME: std::env::var("GRAFANA_DB_USERNAME")
                .expect("Failed to get GRAFANA_DB_USERNAME env"),
            GRAFANA_DB_PASSWORD: std::env::var("GRAFANA_DB_PASSWORD")
                .expect("Failed to get GRAFANA_DB_PASSWORD env"),
            TIMESCALEDB_IMAGE: std::env::var("TIMESCALEDB_IMAGE")
                .expect("Failed to get TIMESCALEDB_IMAGE env"),
            OFELIA_IMAGE: std::env::var("OFELIA_IMAGE").expect("Failed to get OFELIA_IMAGE env"),
            TIMESCALEDB_DATA: std::env::var("TIMESCALEDB_DATA")
                .expect("Failed to get TIMESCALEDB_DATA env"),
            TIMESCALEDB_BACKUPS: std::env::var("TIMESCALEDB_BACKUPS")
                .expect("Failed to get TIMESCALEDB_BACKUPS env"),
            TIMESCALEDB_LOGS: std::env::var("TIMESCALEDB_LOGS")
                .expect("Failed to get TIMESCALEDB_LOGS env"),
            TIMESCALEDB_PGBACKREST_CONFIG: std::env::var("TIMESCALEDB_PGBACKREST_CONFIG")
                .expect("Failed to get TIMESCALEDB_PGBACKREST_CONFIG env"),
            OFELIA_LOGS: std::env::var("OFELIA_LOGS").expect("Failed to get OFELIA_LOGS env"),
            MANUAL_BACKUPS: std::env::var("MANUAL_BACKUPS")
                .expect("Failed to get MANUAL_BACKUPS env"),
            CUSTOM_ENTRYPOINT: std::env::var("CUSTOM_ENTRYPOINT")
                .expect("Failed to get CUSTOM_ENTRYPOINT env"),
            OFELIA_SMTP_HOST: std::env::var("OFELIA_SMTP_HOST")
                .expect("Failed to get OFELIA_SMTP_HOST env"),
            OFELIA_SMTP_PORT: std::env::var("OFELIA_SMTP_PORT")
                .expect("Failed to get OFELIA_SMTP_PORT env"),
            OFELIA_SMTP_USER: std::env::var("OFELIA_SMTP_USER")
                .expect("Failed to get OFELIA_SMTP_USER env"),
            OFELIA_SMTP_PASSWORD: std::env::var("OFELIA_SMTP_PASSWORD")
                .expect("Failed to get OFELIA_SMTP_PASSWORD env"),
            OFELIA_EMAIL_FROM: std::env::var("OFELIA_EMAIL_FROM")
                .expect("Failed to get OFELIA_EMAIL_FROM env"),
            OFELIA_EMAIL_TO: std::env::var("OFELIA_EMAIL_TO")
                .expect("Failed to get OFELIA_EMAIL_TO env"),
        };
        return env;
    })
}
pub fn ONLY_DATABASE() -> bool {
    get_env().ONLY_DATABASE
}

pub fn DATABASE_ADDRESS() -> &'static str {
    get_env().DATABASE_ADDRESS.as_str()
}

pub fn DATABASE_PORT() -> &'static str {
    get_env().DATABASE_PORT.as_str()
}

pub fn POSTGRES_USER() -> &'static str {
    get_env().POSTGRES_USER.as_str()
}

pub fn POSTGRES_PASSWORD() -> &'static str {
    get_env().POSTGRES_PASSWORD.as_str()
}

pub fn POSTGRES_DB() -> &'static str {
    get_env().POSTGRES_DB.as_str()
}

pub fn PG_DATA() -> &'static str {
    get_env().PG_DATA.as_str()
}

pub fn TIMESCALEDB_IMAGE() -> &'static str {
    get_env().TIMESCALEDB_IMAGE.as_str()
}

pub fn OFELIA_IMAGE() -> &'static str {
    get_env().OFELIA_IMAGE.as_str()
}

pub fn TIMESCALEDB_DATA() -> &'static str {
    get_env().TIMESCALEDB_DATA.as_str()
}

pub fn TIMESCALEDB_BACKUPS() -> &'static str {
    get_env().TIMESCALEDB_BACKUPS.as_str()
}

pub fn TIMESCALEDB_LOGS() -> &'static str {
    get_env().TIMESCALEDB_LOGS.as_str()
}

pub fn TIMESCALEDB_PGBACKREST_CONFIG() -> &'static str {
    get_env().TIMESCALEDB_PGBACKREST_CONFIG.as_str()
}

pub fn OFELIA_LOGS() -> &'static str {
    get_env().OFELIA_LOGS.as_str()
}

pub fn MANUAL_BACKUPS() -> &'static str {
    get_env().MANUAL_BACKUPS.as_str()
}

pub fn CUSTOM_ENTRYPOINT() -> &'static str {
    get_env().CUSTOM_ENTRYPOINT.as_str()
}

pub fn OFELIA_SMTP_HOST() -> &'static str {
    get_env().OFELIA_SMTP_HOST.as_str()
}

pub fn OFELIA_SMTP_PORT() -> &'static str {
    get_env().OFELIA_SMTP_PORT.as_str()
}

pub fn OFELIA_SMTP_USER() -> &'static str {
    get_env().OFELIA_SMTP_USER.as_str()
}

pub fn OFELIA_SMTP_PASSWORD() -> &'static str {
    get_env().OFELIA_SMTP_PASSWORD.as_str()
}

pub fn OFELIA_EMAIL_FROM() -> &'static str {
    get_env().OFELIA_EMAIL_FROM.as_str()
}

pub fn OFELIA_EMAIL_TO() -> &'static str {
    get_env().OFELIA_EMAIL_TO.as_str()
}

pub fn GRAFANA_DB_USERNAME() -> &'static str {
    get_env().GRAFANA_DB_USERNAME.as_str()
}

pub fn GRAFANA_DB_PASSWORD() -> &'static str {
    get_env().GRAFANA_DB_PASSWORD.as_str()
}
