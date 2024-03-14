use crate::state::Sessions;
use crate::structs::cloud::cloud_events::events::EventData;
use crate::{
    middlewares::origin_middleware::Origin, structs::cloud::api_cloud_errors::CloudApiErrors,
};
use axum::extract::ConnectInfo;
use axum::{extract::State, http::StatusCode, Json};
use database::db::Db;
use log::error;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use ts_rs::TS;

use super::events_handler::process_event;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpNightlyConnectCloudEvent {
    pub app_id: String,
    pub event: EventData,
}

pub async fn events(
    ConnectInfo(ip): ConnectInfo<SocketAddr>,
    State(db): State<Option<Arc<Db>>>,
    State(sessions): State<Sessions>,
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

    // Process the event
    process_event(request, ip, &db, &sessions).await;

    return Ok(Json(()));
}
