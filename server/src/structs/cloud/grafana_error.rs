use axum::http::StatusCode;
use log::{info, warn};
use openapi::{apis::Error, models::ErrorResponseBody};
use serde::Serialize;
use std::fmt;

pub fn handle_grafana_error<T>(error: Error<T>) -> (StatusCode, String)
where
    T: Serialize + fmt::Debug,
{
    match error {
        Error::Reqwest(err) => {
            warn!("Network error: {}", err);
            (
                StatusCode::BAD_GATEWAY,
                "Network error occurred".to_string(),
            )
        }
        Error::Serde(err) => {
            warn!("Serialization/deserialization error: {}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error processing data".to_string(),
            )
        }
        Error::Io(err) => {
            warn!("I/O error: {}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "I/O error occurred".to_string(),
            )
        }
        Error::ResponseError(response_content) => {
            info!(
                "HTTP error with status {}: {}, entity: {:?}",
                response_content.status, response_content.content, response_content.entity
            );
            let message = match &response_content.entity {
                Some(entity) => {
                    let serialized =
                        serde_json::to_string(entity).unwrap_or_else(|_| "{}".to_string());
                    serde_json::from_str::<ErrorResponseBody>(&serialized).map_or_else(
                        |_| {
                            warn!("Failed to extract ErrorResponseBody, using original content");
                            response_content.content.clone()
                        },
                        |body| body.message,
                    )
                }
                None => response_content.content.clone(),
            };
            let status_code = StatusCode::from_u16(response_content.status.as_u16())
                .unwrap_or_else(|_| {
                    warn!("Failed to convert status code: {}", response_content.status);
                    StatusCode::INTERNAL_SERVER_ERROR
                });
            (status_code, message)
        }
    }
}
