use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{header::ORIGIN, request::Parts, StatusCode},
};
use log::warn;

#[derive(Debug, Clone)]
pub struct Origin(pub Option<String>);

#[async_trait]
impl<B> FromRequestParts<B> for Origin
where
    B: Send,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(req: &mut Parts, _state: &B) -> Result<Self, Self::Rejection> {
        match req.headers.get(ORIGIN) {
            Some(value) =>
            // If anything goes wrong, return empty origin
            {
                match value.to_str() {
                    Ok(origin) => {
                        if origin.is_empty() {
                            warn!("Empty Origin header");
                            return Ok(Origin(None));
                        }

                        Ok(Origin(Some(origin.to_owned())))
                    }
                    Err(e) => {
                        warn!("Failed to parse Origin header {:?}", e);
                        Ok(Origin(None))
                    }
                }
            }
            None => Ok(Origin(None)),
        }
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

    async fn origin_as_body(
        Origin(origin): Origin,
    ) -> Result<Json<Option<String>>, (StatusCode, String)> {
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
        let resp = convert_response::<Option<String>>(response).await.unwrap();

        assert_eq!(resp, Some("https://www.example.com".to_string()));
    }

    #[tokio::test]
    async fn origin_header_empty() {
        let app = app();

        let request = Request::builder()
            .uri("/")
            .header("Origin", "")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();
        let resp = convert_response::<Option<String>>(response).await.unwrap();

        assert_eq!(resp, None);
    }
}
