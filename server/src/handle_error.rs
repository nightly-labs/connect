use axum::response::IntoResponse;
use hyper::StatusCode;
use log::error;
use tower::BoxError;

use crate::errors::NightlyError;

pub async fn handle_error(error: BoxError) -> impl IntoResponse {
    error!("Request error {:?}", error);
    if error.is::<tower::timeout::error::Elapsed>() {
        return (
            StatusCode::REQUEST_TIMEOUT,
            NightlyError::RequestTimeOut.to_string(),
        );
    }
    if error.is::<tower::load_shed::error::Overloaded>() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            NightlyError::ServiceOverload.to_string(),
        );
    }
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!(
            "{} {}",
            NightlyError::UnhandledInternalError.to_string(),
            error
        ),
    )
}
