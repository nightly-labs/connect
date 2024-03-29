#![allow(non_snake_case)]
use once_cell::sync::OnceCell;

#[derive(Debug)]
pub struct ENV {
    pub ENVIRONMENT: String,
    pub JWT_SECRET: String,
    pub JWT_PUBLIC_KEY: String,
    pub ONLY_RELAY_SERVICE: bool,
    pub NONCE: String,
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
