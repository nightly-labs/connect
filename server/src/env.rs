#![allow(non_snake_case)]
use once_cell::sync::OnceCell;

#[derive(Debug)]
pub struct ENV {
    pub ENVIRONMENT: String,
    pub JWT_SECRET: String,
    pub JWT_PUBLIC_KEY: String,
    pub ONLY_RELAY_SERVICE: bool,
    pub NONCE: String,
    pub MAILER_ADDRESS: String,
    pub MAILER_PASSWORD: String,
    pub GRAFANA_BASE_PATH: String,
    pub GF_SECURITY_ADMIN_USER: String,
    pub GF_SECURITY_ADMIN_PASSWORD: String,
}
pub fn get_env() -> &'static ENV {
    static INSTANCE: OnceCell<ENV> = OnceCell::new();

    INSTANCE.get_or_init(|| {
        dotenvy::from_filename(".env").ok();
        let ENVIRONMENT = std::env::var("ENV").expect("Failed to get ENV env");
        let ENVIRONMENT = ENVIRONMENT.as_str();

        let env = ENV {
            ENVIRONMENT: ENVIRONMENT.to_owned(),
            JWT_SECRET: std::env::var("JWT_SECRET").expect("JWT_SECRET env not set"),
            JWT_PUBLIC_KEY: std::env::var("JWT_PUBLIC_KEY").expect("JWT_PUBLIC_KEY env not set"),
            ONLY_RELAY_SERVICE: std::env::var("ONLY_RELAY_SERVICE")
                .expect("Failed to get ONLY_RELAY_SERVICE env")
                .eq_ignore_ascii_case("true"),
            NONCE: std::env::var("NONCE").expect("Failed to get NONCE env"),
            MAILER_ADDRESS: std::env::var("MAILER_ADDRESS")
                .expect("Failed to get MAILER_ADDRESS env"),
            MAILER_PASSWORD: std::env::var("MAILER_PASSWORD")
                .expect("Failed to get MAILER_PASSWORD env"),
            GRAFANA_BASE_PATH: std::env::var("GRAFANA_BASE_PATH")
                .expect("Failed to get GRAFANA_BASE_PATH env"),
            GF_SECURITY_ADMIN_USER: std::env::var("GF_SECURITY_ADMIN_USER")
                .expect("Failed to get GF_SECURITY_ADMIN_USER env"),
            GF_SECURITY_ADMIN_PASSWORD: std::env::var("GF_SECURITY_ADMIN_PASSWORD")
                .expect("Failed to get GF_SECURITY_ADMIN_PASSWORD env"),
        };
        return env;
    })
}

pub fn is_env_production() -> bool {
    ENVIRONMENT() == "PROD"
}
pub fn ENVIRONMENT() -> &'static str {
    get_env().ENVIRONMENT.as_str()
}
pub fn JWT_SECRET() -> &'static str {
    get_env().JWT_SECRET.as_str()
}
pub fn JWT_PUBLIC_KEY() -> &'static str {
    get_env().JWT_PUBLIC_KEY.as_str()
}
pub fn ONLY_RELAY_SERVICE() -> bool {
    get_env().ONLY_RELAY_SERVICE
}
pub fn NONCE() -> &'static str {
    get_env().NONCE.as_str()
}
pub fn MAILER_ADDRESS() -> &'static str {
    get_env().MAILER_ADDRESS.as_str()
}
pub fn MAILER_PASSWORD() -> &'static str {
    get_env().MAILER_PASSWORD.as_str()
}
pub fn GRAFANA_BASE_PATH() -> &'static str {
    get_env().GRAFANA_BASE_PATH.as_str()
}
pub fn GF_SECURITY_ADMIN_USER() -> &'static str {
    get_env().GF_SECURITY_ADMIN_USER.as_str()
}
pub fn GF_SECURITY_ADMIN_PASSWORD() -> &'static str {
    get_env().GF_SECURITY_ADMIN_PASSWORD.as_str()
}
