use super::domain_verification_status::DomainVerificationStatus;
use crate::tables::registered_app::table_struct::DbRegisteredApp;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct WhitelistedDomain {
    pub domain: String,
    pub status: DomainVerificationStatus,
}

impl DbRegisteredApp {
    pub fn get_whitelisted_domains(&self) -> Vec<WhitelistedDomain> {
        self.whitelisted_domains
            .iter()
            .map(|domain| WhitelistedDomain {
                domain: domain.clone(),
                status: DomainVerificationStatus::Verified,
            })
            .collect()
    }
}
