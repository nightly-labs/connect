use axum::{
    extract::Path,
    http::StatusCode,
    response::{AppendHeaders, IntoResponse},
};
use tokio::fs;

use crate::errors::NightlyError;

pub async fn get_image(
    Path((slug, id)): Path<(String, String)>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // TODO add other formats
    // TODO probably move to CDN
    // Read from disk
    let images_path = std::env::var("IMAGES_PATH").unwrap_or_default();
    let path = shellexpand::tilde(&images_path);
    let img = match fs::read_to_string(format!("{path}/{slug}/{id}")).await {
        Ok(img) => img,
        Err(_) => return Err((StatusCode::NOT_FOUND, NightlyError::NotFound.to_string())),
    };
    // Return image
    return Ok((
        AppendHeaders([(axum::http::header::CONTENT_TYPE, "image/svg+xml")]),
        img,
    ));
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_get_image() {
        dotenvy::dotenv().ok();
        let slug = "nightly".to_string();
        let id = "default.svg".to_string();
        let img = super::get_image(axum::extract::Path((slug, id))).await;
        assert!(img.is_ok())
    }
}
