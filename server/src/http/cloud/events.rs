use crate::{
    middlewares::origin_middleware::Origin,
    structs::{api_cloud_errors::CloudApiErrors, cloud_events::events::EventData},
};
use axum::{extract::State, http::StatusCode, Json};
use database::db::Db;
use log::error;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpNightlyConnectCloudEvent {
    pub app_id: String,
    pub event: EventData,
}

pub async fn events(
    State(db): State<Option<Arc<Db>>>,
    Origin(origin): Origin,
    Json(request): Json<HttpNightlyConnectCloudEvent>,
) -> Result<Json<()>, (StatusCode, String)> {
    // Db connection has already been checked in the middleware
    let db = db.as_ref().ok_or((
        StatusCode::INTERNAL_SERVER_ERROR,
        CloudApiErrors::CloudFeatureDisabled.to_string(),
    ))?;

    // Check if origin and app_id match in the database
    match db.get_registered_app_by_app_id(&origin).await {
        Ok(Some(app)) => {
            app.whitelisted_domains
                .iter()
                .find(|&d| d == &origin)
                .ok_or((
                    StatusCode::FORBIDDEN,
                    CloudApiErrors::UnauthorizedOriginError.to_string(),
                ))?;
        }
        Ok(None) => {
            return Err((
                StatusCode::BAD_REQUEST,
                CloudApiErrors::AppDoesNotExist.to_string(),
            ));
        }
        Err(e) => {
            error!("Database error: {}", e);
            return Err((
                StatusCode::BAD_REQUEST,
                CloudApiErrors::DatabaseError.to_string(),
            ));
        }
    }

    // TODO process event

    return Ok(Json(()));
}
