use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[derive(Debug, Serialize, PartialEq, Eq, Deserialize, Clone, Display, EnumString)]
pub enum AuthTokenType {
    Access,  // Usually short-lived, used to access protected resources
    Refresh, // Usually long-lived, used to obtain new access tokens
}
