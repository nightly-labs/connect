use std::sync::Arc;

use crate::structs::requests_structs_filters::registered_app::RegisteredApp;
use axum::{extract::State, http::StatusCode, Json};
use database::db::Db;
use log::error;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpGetRegisteredAppsRequest {
    pub team_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct HttpGetRegisteredAppsResponse {
    pub apps: Vec<RegisteredApp>,
}

pub async fn get_registered_apps(
    State(db): State<Arc<Db>>,
    Json(request): Json<HttpGetRegisteredAppsRequest>,
) -> Result<Json<HttpGetRegisteredAppsResponse>, (StatusCode, String)> {
    let _ = db
        .get_team_by_team_id(None, &request.team_id)
        .await
        .map_err(|e| {
            error!("Failed to get team: {:?}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get team".to_string(),
            )
        })?;

    let apps = match db.get_registered_apps_by_team_id(&request.team_id).await {
        Ok(apps) => apps.into_iter().map(|app| app.into()).collect(),
        Err(e) => {
            error!("Failed to get registered apps: {:?}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to get registered apps".to_string(),
            ));
        }
    };

    return Ok(Json(HttpGetRegisteredAppsResponse { apps }));
}
