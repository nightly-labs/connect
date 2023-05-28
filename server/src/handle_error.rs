use axum::response::IntoResponse;
use hyper::StatusCode;
use tower::BoxError;

use crate::errors::ApiErrors;

pub async fn handle_error(error: BoxError) -> impl IntoResponse {
    if error.is::<tower::timeout::error::Elapsed>() {
        return (
            StatusCode::REQUEST_TIMEOUT,
            ApiErrors::RequestTimeOut.as_str(),
        );
    }

    if error.is::<tower::load_shed::error::Overloaded>() {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            ApiErrors::ServiceOverload.as_str(),
        );
    }

    (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("{} {}", ApiErrors::UnhandledInternalError.as_str(), error),
    )
}
