use r_cache::cache::Cache;

pub type ApiSessionsCache = Cache<String, SessionCache>;

#[derive(Debug, Clone)]
pub enum SessionCache {
    VerifyRegister(RegisterVerification),
}

#[derive(Debug, Clone)]
pub struct RegisterVerification {
    pub email: String,
    pub hashed_password: String,
    pub code: String,
    pub created_at: u64,
}
