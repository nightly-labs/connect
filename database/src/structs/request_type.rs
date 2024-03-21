use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use ts_rs::TS;

#[derive(Debug, Display, Clone, PartialEq, Eq, Serialize, Deserialize, TS, EnumString)]
#[ts(export)]
pub enum RequestType {
    SignMessage,
    SignTransaction,
    SignAndSendTransaction,
    ChangeWallet,
    ChangeNetwork,
}
