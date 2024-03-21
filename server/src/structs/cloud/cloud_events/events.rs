use super::event_types::{
    app_connect_event::AppConnectEvent, app_disconnect_event::AppDisconnectEvent,
    change_network_event::ChangeNetworkEvent, change_wallet_event::ChangeWalletEvent,
    client_connect_event::ClientConnectEvent,
    client_connect_resolve_event::ClientConnectResolveEvent,
    client_disconnect_event::ClientDisconnectEvent,
    sign_and_send_transaction_event::SignAndSendTransactionEvent,
    sign_and_send_transaction_resolve_event::SignAndSendTransactionResolveEvent,
    sign_message_event::SignMessageEvent, sign_message_resolve_event::SignMessageResolveEvent,
    sign_transaction_event::SignTransactionEvent,
    sign_transaction_resolve_event::SignTransactionResolveEvent,
};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(tag = "type")]
pub enum EventData {
    AppConnect(AppConnectEvent),
    AppDisconnect(AppDisconnectEvent),
    ClientConnect(ClientConnectEvent),
    ClientConnectResolve(ClientConnectResolveEvent),
    ClientDisconnect(ClientDisconnectEvent),
    SignMessage(SignMessageEvent),
    SignMessageResolve(SignMessageResolveEvent),
    SignTransaction(SignTransactionEvent),
    SignTransactionResolve(SignTransactionResolveEvent),
    SignAndSendTransaction(SignAndSendTransactionEvent),
    SignAndSendTransactionResolve(SignAndSendTransactionResolveEvent),
    ChangeNetwork(ChangeNetworkEvent),
    ChangeWallet(ChangeWalletEvent),
}
