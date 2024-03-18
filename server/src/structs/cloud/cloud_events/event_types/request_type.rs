use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub enum RequestType {
    SignMessage,
    SignTransaction,
    SignAndSendTransaction,
    ChangeWallet,
    ChangeNetwork,
}
