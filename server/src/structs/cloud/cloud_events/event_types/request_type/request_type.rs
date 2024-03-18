use super::{message_to_sign::MessageToSign, transaction_to_sign::TransactionToSign};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub enum RequestType {
    SignMessages(Vec<MessageToSign>),
    SignTransactions(Vec<TransactionToSign>),
    SignAndSendTransactions(Vec<TransactionToSign>),
    ChangeWallet,
    ChangeNetwork,
}
