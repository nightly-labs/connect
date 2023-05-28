use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::structs::common::{Network, Version};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct GetInfoRequest {
  #[serde(rename = "responseId")]
  pub response_id: String,
  #[serde(rename = "sessionId")]
  pub session_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct GetInfoResponse {
  #[serde(rename = "responseId")]
  pub response_id: String,
  #[serde(rename = "appName")]
  pub app_name: String,
  pub network: Network,
  pub version: Version,
  #[serde(rename = "appDescription")]
  pub app_description: Option<String>,
  #[serde(rename = "appIcon")]
  pub app_icon: Option<String>,
  #[serde(rename = "additionalInfo")]
  pub additional_info: Option<String>,
}
