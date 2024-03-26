use crate::middlewares::auth_middleware::UserId;
use crate::structs::cloud::api_cloud_errors::CloudApiErrors;
use crate::structs::cloud::app_event::AppEvent;
use crate::utils::{custom_validate_optional_pagination_cursor, custom_validate_uuid};
use axum::Extension;
use axum::{extract::State, http::StatusCode, Json};
use database::db::Db;
use database::structs::pagination_cursor::PaginationCursor;
use garde::Validate;
use log::error;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS, Validate)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpGetAppEventsEventRequest {
    #[garde(custom(custom_validate_uuid))]
    pub app_id: String,
    #[garde(custom(custom_validate_optional_pagination_cursor))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination_cursor: Option<PaginationCursor>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpGetAppEventsResponse {
    pub events: Vec<AppEvent>,
    pub cursor: Option<PaginationCursor>,
}

pub async fn events(
    State(db): State<Option<Arc<Db>>>,
    Extension(user_id): Extension<UserId>,
    Json(request): Json<HttpGetAppEventsEventRequest>,
) -> Result<Json<HttpGetAppEventsResponse>, (StatusCode, String)> {
    // Db connection has already been checked in the middleware
    let db = db.as_ref().ok_or((
        StatusCode::INTERNAL_SERVER_ERROR,
        CloudApiErrors::CloudFeatureDisabled.to_string(),
    ))?;

    // Check if user has sufficient permissions
    match db
        .get_privilege_by_user_id_and_app_id(&user_id, &request.app_id)
        .await
    {
        Ok(Some(_)) => {
            // Get events
            let (events, cursor) = match db
                .get_events_by_app_id(request.pagination_cursor, &request.app_id)
                .await
            {
                Ok((events, cursor)) => {
                    let events = events.into_iter().map(|e| e.into()).collect();
                    (events, cursor)
                }
                Err(e) => {
                    error!("Failed to get events by app id: {:?}", e);
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        CloudApiErrors::InternalServerError.to_string(),
                    ));
                }
            };

            return Ok(Json(HttpGetAppEventsResponse { events, cursor }));
        }
        Ok(None) => {
            return Err((
                StatusCode::FORBIDDEN,
                CloudApiErrors::InsufficientPermissions.to_string(),
            ));
        }
        Err(e) => {
            error!("Failed to get privilege by user id and app id: {:?}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                CloudApiErrors::InternalServerError.to_string(),
            ));
        }
    }
}
