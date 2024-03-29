use garde::rules::pattern::regex::Regex;
use once_cell::sync::Lazy;

pub const TEAMS_AMOUNT_LIMIT_PER_USER: usize = 10;
pub const REGISTERED_APPS_LIMIT_PER_TEAM: usize = 20;
pub const USERS_AMOUNT_LIMIT_PER_TEAM: usize = 50;

// Name must be 3-30 characters long and include only alphanumeric characters, underscores, or slashes.
pub static NAME_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[a-zA-Z0-9_-]{3,30}$").expect("Regex creation failed"));

pub struct PasswordValidator {
    pub re: Regex,
    pub error: String,
}

pub static REGISTER_PASSWORD_VALIDATOR: Lazy<Vec<PasswordValidator>> = Lazy::new(|| {
    vec![
        PasswordValidator {
            re: Regex::new(r"[a-z]").expect("Regex creation failed"),
            error: "Password do not contain lowercase letter".to_string(),
        },
        PasswordValidator {
            re: Regex::new(r"[A-Z]").expect("Regex creation failed"),
            error: "Password do not contain uppercase letter".to_string(),
        },
        PasswordValidator {
            re: Regex::new(r"\d").expect("Regex creation failed"),
            error: "Password do not contain digit".to_string(),
        },
        PasswordValidator {
            re: Regex::new(r".{8,40}").expect("Regex creation failed"),
            error: "Password is too short".to_string(),
        },
    ]
});
