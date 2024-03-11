use super::{auth_token_type::AuthTokenType, AuthToken};
use crate::env::JWT_PUBLIC_KEY;
use axum::{
    extract::{ConnectInfo, Request},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use std::net::SocketAddr;

pub type UserId = String;

pub async fn access_auth_middleware(
    ConnectInfo(ip): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let auth_header = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|header| {
            let header = header.to_str().ok()?;
            Some(header.to_string())
        });

    let auth_token = match auth_header {
        Some(auth_header) => {
            let token = auth_header.replace("Bearer ", "");
            // Decode and validate the token
            let decoded = match AuthToken::decode(&token, JWT_PUBLIC_KEY(), ip) {
                Ok(decoded) => decoded,
                Err(e) => return Err((StatusCode::UNAUTHORIZED, e.to_string())),
            };
            if decoded.token_type != AuthTokenType::Access {
                return Err((StatusCode::UNAUTHORIZED, "Invalid token type".to_string()));
            }
            decoded
            // do something with the token
        }
        None => return Err((StatusCode::UNAUTHORIZED, "No auth token".to_string())),
    };
    // Insert the user_id into the request extensions
    req.extensions_mut()
        .insert(auth_token.user_id.clone() as UserId);
    req.extensions_mut().insert(auth_token);

    let response = next.run(req).await;
    Ok(response)
}
