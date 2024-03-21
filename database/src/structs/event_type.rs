use serde::{Deserialize, Serialize};
use sqlx::Type;
use ts_rs::TS;

#[derive(Clone, Debug, Eq, PartialEq, Type, Serialize, Deserialize, TS)]
#[ts(export)]
#[sqlx(type_name = "event_type_enum")]
pub enum EventType {
    AppConnect,
    AppDisconnect,
    ClientConnect,
    ClientDisconnect,
    SignMessage,
    SignTransaction,
    SignAndSendTransaction,
    ChangeWallet,
    ChangeNetwork,
}
