use crate::{ip_geolocation::GeolocationRequester, structs::session_cache::ApiSessionsCache};
use axum::extract::FromRef;
use database::db::Db;
use r_cache::cache::Cache;
use std::{sync::Arc, time::Duration};
use tokio::task;

#[derive(Clone, FromRef)]
pub struct CloudState {
    pub db: Arc<Db>,
    pub geo_location: Arc<GeolocationRequester>,
    pub sessions_cache: Arc<ApiSessionsCache>,
}

impl CloudState {
    pub fn new(db: Arc<Db>, geo_location: Arc<GeolocationRequester>) -> Self {
        let sessions_cache = get_new_session();

        Self {
            db,
            geo_location,
            sessions_cache,
        }
    }
}

pub fn get_new_session() -> Arc<ApiSessionsCache> {
    let add_new_passkey_sessions = Arc::new(Cache::new(Some(Duration::from_secs(300))));
    task::spawn({
        let cache = Arc::clone(&add_new_passkey_sessions);
        async move {
            loop {
                tokio::time::sleep(Duration::from_secs(300)).await;
                cache.remove_expired();
            }
        }
    });
    add_new_passkey_sessions
}
