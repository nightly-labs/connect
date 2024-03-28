use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{header::ORIGIN, request::Parts, StatusCode},
};

#[derive(Debug, Clone)]
pub struct Origin(pub String);

#[async_trait]
impl<B> FromRequestParts<B> for Origin
where
    B: Send,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(req: &mut Parts, _state: &B) -> Result<Self, Self::Rejection> {
        let origin = req
            .headers
            .get(ORIGIN)
            .and_then(|value| value.to_str().ok())
            .map(|s| s.to_owned())
            .ok_or_else(|| {
                println!("Origin header is required");
                (
                    StatusCode::BAD_REQUEST,
                    "Origin header is required".to_string(),
                )
            })?;

        Ok(Origin(origin))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::test_utils::convert_response;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        routing::get,
        Json, Router,
    };
    use tower::ServiceExt;

    async fn origin_as_body(Origin(origin): Origin) -> Result<Json<String>, (StatusCode, String)> {
        Ok(Json(origin))
    }

    fn app() -> Router {
        Router::new().route("/", get(origin_as_body))
    }

    #[tokio::test]
    async fn origin_header() {
        let app = app();

        let request = Request::builder()
            .uri("/")
            .header("Origin", "https://www.example.com")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        let resp = convert_response::<String>(response).await.unwrap();

        assert_eq!(resp, "https://www.example.com");
    }

    #[tokio::test]
    async fn missing_origin_header() {
        let app = app();

        let request = Request::builder().uri("/").body(Body::empty()).unwrap();

        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let err = convert_response::<String>(response).await.unwrap_err();
        assert_eq!(err.to_string(), "Origin header is required");
    }
}
