use crate::state::ServerState;
use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::IntoResponse,
};

pub async fn db_cloud_middleware(
    State(state): State<ServerState>,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
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
