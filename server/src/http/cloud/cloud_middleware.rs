use crate::state::ServerState;
use axum::{extract::Request, http::StatusCode, middleware::Next, response::IntoResponse};
use std::sync::Arc;

pub async fn db_cloud_middleware(
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // Extract the state from the request extensions
    let state = match req.extensions().get::<Arc<ServerState>>() {
        Some(state) => state,
        None => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Corrupted server state".to_string(),
            ))
        }
    };

    // Check if the database is connected
    if state.db.is_some() {
        // If the database is connected, pass the request to the next middleware or handler
        Ok(next.run(req).await)
    } else {
        // If the database is not connected, return an error response
        Err((
            StatusCode::FORBIDDEN,
            "Cloud endpoints are disabled".to_string(),
        ))
    }
}
