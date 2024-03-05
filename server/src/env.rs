#![allow(non_snake_case)]
use once_cell::sync::OnceCell;
use rand::{distributions::Alphanumeric, thread_rng, Rng};

#[derive(Debug)]
pub struct ENV {
    pub ENVIRONMENT: String,
    pub JWT_SECRET: String,
}
pub fn get_env() -> &'static ENV {
    static INSTANCE: OnceCell<ENV> = OnceCell::new();

    INSTANCE.get_or_init(|| {
        dotenvy::from_filename(".env").ok();
        let ENVIRONMENT = std::env::var("ENVIRONMENT").expect("Failed to get ENVIRONMENT env");
        let ENVIRONMENT = ENVIRONMENT.as_str();

        let env = ENV {
            ENVIRONMENT: ENVIRONMENT.to_owned(),
            JWT_SECRET: match ENVIRONMENT {
                "PROD" => std::env::var("JWT_SECRET").expect("JWT_SECRET env not set"),
                "DEV" => {
                    let rand_string: String = thread_rng()
                        .sample_iter(&Alphanumeric)
                        .take(6)
                        .map(char::from)
                        .collect();

                    std::env::var("JWT_SECRET").expect("JWT_SECRET env not set")
                        + rand_string.as_str()
                }
                _ => panic!("Invalid ENVIRONMENT"),
            },
        };
        return env;
    })
}

pub fn ENVIRONMENT() -> &'static str {
    get_env().ENVIRONMENT.as_str()
}
pub fn JWT_SECRET() -> &'static str {
    get_env().JWT_SECRET.as_str()
}
