use crate::{structs::wallet_metadata::WalletMetadata, wallets::WALLETS_METADATA};
use axum::{http::StatusCode, Json};

pub async fn get_wallets_metadata() -> Result<Json<Vec<WalletMetadata>>, (StatusCode, String)> {
    Ok(Json(WALLETS_METADATA.to_vec()))
}
