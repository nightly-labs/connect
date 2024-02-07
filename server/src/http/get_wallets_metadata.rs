use crate::structs::wallet_metadata::WalletMetadata;
use axum::{extract::State, http::StatusCode, Json};
use std::{ops::Deref, sync::Arc};

pub async fn get_wallets_metadata(
    State(wallets_metadata): State<Arc<Vec<WalletMetadata>>>,
) -> Result<Json<Vec<WalletMetadata>>, (StatusCode, String)> {
    Ok(Json(wallets_metadata.deref().clone()))
}
