use crate::{
    ip_geolocation::GeolocationRequester,
    mailer::{entry::run_mailer, mailer::Mailer},
    structs::session_cache::ApiSessionsCache,
};
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
    pub mailer: Arc<Mailer>,
}

impl CloudState {
    pub async fn new() -> Self {
        let sessions_cache = get_new_session();
        let db_arc = Arc::new(Db::connect_to_the_pool().await);
        let geo_loc_requester = Arc::new(GeolocationRequester::new().await);
        let mailer = Arc::new(run_mailer().await.unwrap());

        Self {
            db: db_arc,
            geo_location: geo_loc_requester,
            sessions_cache,
            mailer,
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
