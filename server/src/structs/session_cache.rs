use r_cache::cache::Cache;

pub type ApiSessionsCache = Cache<String, SessionCache>;

#[derive(Debug, Clone)]
pub enum SessionCache {
    VerifyRegister(RegisterVerification),
    ResetPassword(ResetPasswordVerification),
    VerifyDomain(DomainVerification),
}

pub enum SessionsCacheKey {
    RegisterVerification(String),      // user email
    ResetPasswordVerification(String), // user email
    DomainVerification(String),        // domain name
}

impl SessionsCacheKey {
    pub fn to_string(&self) -> String {
        match self {
            SessionsCacheKey::RegisterVerification(email) => format!("reg_ver_{}", email),
            SessionsCacheKey::ResetPasswordVerification(email) => format!("pass_res_{}", email),
            SessionsCacheKey::DomainVerification(domain_name) => format!("dom_ver_{}", domain_name),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RegisterVerification {
    pub email: String,
    pub hashed_password: String,
    pub code: String,
    pub created_at: u64,
}

#[derive(Debug, Clone)]
pub struct ResetPasswordVerification {
    pub email: String,
    pub hashed_new_password: String,
    pub code: String,
    pub created_at: u64,
}

#[derive(Debug, Clone)]
pub struct DomainVerification {
    pub domain_name: String,
    pub code: String,
    pub created_at: u64,
}
