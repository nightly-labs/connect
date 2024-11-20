use r_cache::cache::Cache;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub type ApiSessionsCache = Cache<String, SessionCache>;

#[derive(Debug, Clone)]
pub enum SessionCache {
    VerifyRegister(RegisterVerification),
    ResetPassword(ResetPasswordVerification),
    VerifyPasskeyRegister(PasskeyVerification),
    ResetPasskey(ResetPasskeyVerification),
    Passkey2FA(Passkey2FAVerification),
    VerifyAddPasskey(AddPasskeyVerification),
    PasskeyLogin(PasskeyLoginVerification),
    DeleteAccount(DeleteAccountVerification),
}

pub enum SessionsCacheKey {
    RegisterVerification(String),      // user email
    ResetPasswordVerification(String), // user email
    PasskeyVerification(String),       // user email
    ResetPasskeyVerification(String),  // user email
    Passkey2FA(String),                // user id
    AddPasskey(String),                // user id
    PasskeyLogin(String),              // user email
    DeleteAccount(String),             // user email
}

impl SessionsCacheKey {
    pub fn to_string(&self) -> String {
        match self {
            SessionsCacheKey::RegisterVerification(email) => format!("reg_ver_{}", email),
            SessionsCacheKey::ResetPasswordVerification(email) => format!("pass_res_{}", email),
            SessionsCacheKey::PasskeyVerification(email) => format!("pass_reg_{}", email),
            SessionsCacheKey::ResetPasskeyVerification(email) => format!("pass_res_{}", email),
            SessionsCacheKey::Passkey2FA(user_id) => format!("pass_chal_{}", user_id),
            SessionsCacheKey::AddPasskey(email) => format!("add_pass_{}", email),
            SessionsCacheKey::PasskeyLogin(email) => format!("pass_login_{}", email),
            SessionsCacheKey::DeleteAccount(email) => format!("del_acc_{}", email),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub enum VerificationAction {
    RegisterPassword,
    RegisterPasskey,
    ResetPassword,
    ResetPasskey,
    DeleteAccount,
}

impl VerificationAction {
    pub fn to_session_key(&self, user_data: String) -> SessionsCacheKey {
        match self {
            VerificationAction::RegisterPassword => {
                SessionsCacheKey::RegisterVerification(user_data)
            }
            VerificationAction::RegisterPasskey => SessionsCacheKey::PasskeyVerification(user_data),
            VerificationAction::ResetPassword => {
                SessionsCacheKey::ResetPasswordVerification(user_data)
            }
            VerificationAction::ResetPasskey => {
                SessionsCacheKey::ResetPasskeyVerification(user_data)
            }
            VerificationAction::DeleteAccount => SessionsCacheKey::DeleteAccount(user_data),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RegisterVerification {
    pub email: String,
    pub verification_code: String,
    pub authentication_code: Option<String>,
    pub created_at: u64,
}

#[derive(Debug, Clone)]
pub struct ResetPasswordVerification {
    pub email: String,
    pub verification_code: String,
    pub authentication_code: Option<String>,
    pub created_at: u64,
}

#[derive(Debug, Clone)]
pub struct PasskeyVerification {
    pub email: String,
    pub verification_code: String,
    pub authentication_code: Option<String>,
    pub passkey_registration_state: webauthn_rs::prelude::PasskeyRegistration,
    pub created_at: u64,
}

#[derive(Debug, Clone)]
pub struct ResetPasskeyVerification {
    pub email: String,
    pub verification_code: String,
    pub authentication_code: Option<String>,
    pub passkey_registration_state: webauthn_rs::prelude::PasskeyRegistration,
    pub created_at: u64,
}

#[derive(Debug, Clone)]
pub struct AddPasskeyVerification {
    pub user_id: String,
    pub passkey_registration_state: webauthn_rs::prelude::PasskeyRegistration,
    pub created_at: u64,
}

#[derive(Debug, Clone)]
pub struct Passkey2FAVerification {
    pub email: String,
    pub passkey_verification_state: webauthn_rs::prelude::PasskeyAuthentication,
    pub created_at: u64,
}

#[derive(Debug, Clone)]
pub struct PasskeyLoginVerification {
    pub email: String,
    pub passkey_verification_state: webauthn_rs::prelude::PasskeyAuthentication,
    pub created_at: u64,
}

#[derive(Debug, Clone)]
pub struct DeleteAccountVerification {
    pub email: String,
    pub verification_code: String,
    pub authentication_code: Option<String>,
    pub created_at: u64,
}
