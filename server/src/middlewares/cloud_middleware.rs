use crate::state::ServerState;
use axum::extract::State;
use axum::{extract::Request, http::StatusCode, middleware::Next, response::IntoResponse};

pub async fn cloud_middleware(
    State(server_state): State<ServerState>,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    // if let Some(cloud_state) = &server_state.cloud_state {
    //     req.extensions_mut().insert(cloud_state.db.clone());
    //     req.extensions_mut()
    //         .insert(cloud_state.geo_location.clone());
    // } else {
    //     return Err((
    //         StatusCode::FORBIDDEN,
    //         "Cloud endpoints are disabled".to_string(),
    //     ));
    // }

    if let None = server_state.db {
        return Err((
            StatusCode::FORBIDDEN,
            "Cloud endpoints are disabled".to_string(),
        ));
    }

    let response = next.run(req).await;
    Ok(response)
}
