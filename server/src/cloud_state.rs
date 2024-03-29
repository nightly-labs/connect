use crate::ip_geolocation::GeolocationRequester;
use axum::extract::FromRef;
use database::db::Db;
use std::sync::Arc;

#[derive(Clone, FromRef)]
pub struct CloudState {
    pub db: Arc<Db>,
    pub geo_location: Arc<GeolocationRequester>,
}

impl CloudState {
    pub fn new(db: Arc<Db>, geo_location: Arc<GeolocationRequester>) -> Self {
        Self { db, geo_location }
    }
}
