use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumIter, EnumString};
use ts_rs::TS;

#[derive(
    Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS, Display, EnumIter, EnumString,
)]
#[ts(export)]
#[allow(non_camel_case_types)]
pub enum WalletType {
    mobile,    // Only mobile
    extension, // Only extension
    hybrid,    // Extension and mobile
}
