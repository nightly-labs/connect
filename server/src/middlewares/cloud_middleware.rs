use crate::state::ServerState;
use axum::extract::State;
use axum::{extract::Request, http::StatusCode, middleware::Next, response::IntoResponse};

pub async fn cloud_middleware(
    State(server_state): State<ServerState>,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if let None = server_state.cloud_state {
        return Err((
            StatusCode::FORBIDDEN,
            "Cloud endpoints are disabled".to_string(),
        ));
    }

    let response = next.run(req).await;
    Ok(response)
}
