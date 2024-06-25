use super::events_handler::process_event;
use crate::http::cloud::utils::extract_domain_name;
use crate::ip_geolocation::GeolocationRequester;
use crate::state::Sessions;
use crate::structs::cloud::cloud_events::events::EventData;
use crate::{
    middlewares::origin_middleware::Origin, structs::cloud::api_cloud_errors::CloudApiErrors,
};
use axum::extract::ConnectInfo;
use axum::{extract::State, http::StatusCode, Json};
use database::db::Db;
use log::{error, warn};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::sync::Arc;
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, TS)]
#[ts(export)]
#[serde(rename_all = "camelCase")]
pub struct HttpNightlyConnectCloudEvent {
    pub app_id: String,
    pub network: String,
    pub event: EventData,
}

pub async fn events(
    ConnectInfo(ip): ConnectInfo<SocketAddr>,
    State(db): State<Arc<Db>>,
    State(geo_loc_requester): State<Arc<GeolocationRequester>>,
    State(sessions): State<Sessions>,
    Origin(origin): Origin,
    Json(request): Json<HttpNightlyConnectCloudEvent>,
) -> Result<Json<()>, (StatusCode, String)> {
    println!("Received event: {:?}", request);
    println!("Origin: {:?}", origin);
    // Check if origin was provided
    let origin = origin.ok_or((
        StatusCode::FORBIDDEN,
        CloudApiErrors::OriginHeaderRequired.to_string(),
    ))?;

    let domain_name = extract_domain_name(&origin).map_err(|err| {
        warn!("{}", err);
        (
            StatusCode::BAD_REQUEST,
            CloudApiErrors::InvalidDomainName.to_string(),
        )
    })?;

    println!("Domain name: {:?}", domain_name);

    // Check if origin and app_id match in the database
    match db.get_registered_app_by_app_id(&request.app_id).await {
        Ok(Some(app)) => {
            app.whitelisted_domains
                .iter()
                .find(|&d| d == &domain_name)
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
    process_event(request, ip, &db, &geo_loc_requester, &sessions).await;

    return Ok(Json(()));
}
