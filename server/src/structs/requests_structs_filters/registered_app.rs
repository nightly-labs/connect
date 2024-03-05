use database::{
    structs::subscription::Subscription, tables::registered_app::table_struct::DbRegisteredApp,
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct RegisteredApp {
    pub app_id: String,
    pub app_name: String,
    pub whitelisted_domains: Vec<String>,
    pub subscription: Option<Subscription>,
    pub ack_public_keys: Vec<String>,
    pub email: Option<String>,
    pub registration_timestamp: u64,
}

impl From<DbRegisteredApp> for RegisteredApp {
    fn from(db_registered_app: DbRegisteredApp) -> Self {
        RegisteredApp {
            app_id: db_registered_app.app_id,
            app_name: db_registered_app.app_name,
            whitelisted_domains: db_registered_app.whitelisted_domains,
            subscription: db_registered_app.subscription,
            ack_public_keys: db_registered_app.ack_public_keys,
            email: db_registered_app.email,
            registration_timestamp: db_registered_app.registration_timestamp,
        }
    }
}
