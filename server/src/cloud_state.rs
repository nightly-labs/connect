use crate::{
    env::ENVIRONMENT,
    ip_geolocation::GeolocationRequester,
    mailer::{entry::run_mailer, mailer::Mailer},
    structs::session_cache::ApiSessionsCache,
};
use axum::extract::FromRef;
use database::db::Db;
use hickory_resolver::{
    name_server::{GenericConnector, TokioRuntimeProvider},
    AsyncResolver, TokioAsyncResolver,
};
use r_cache::cache::Cache;
use reqwest::Url;
use std::{sync::Arc, time::Duration};
use tokio::task;
use webauthn_rs::{Webauthn, WebauthnBuilder};

pub type DnsResolver = AsyncResolver<GenericConnector<TokioRuntimeProvider>>;

#[derive(Clone, FromRef)]
pub struct CloudState {
    pub db: Arc<Db>,
    pub geo_location: Arc<GeolocationRequester>,
    pub sessions_cache: Arc<ApiSessionsCache>,
    pub mailer: Arc<Mailer>,
    pub dns_resolver: Arc<TokioAsyncResolver>,
    pub webauthn: Arc<Webauthn>,
}

impl CloudState {
    pub async fn new() -> Self {
        let sessions_cache = get_new_session();
        let db_arc = Arc::new(Db::connect_to_the_pool().await);
        let geo_loc_requester = Arc::new(GeolocationRequester::new().await);
        let mailer = Arc::new(run_mailer().await.unwrap());
        let dns_resolver = Arc::new(TokioAsyncResolver::tokio_from_system_conf().unwrap());

        // Passkey
        let rp_id = match ENVIRONMENT() {
            "DEV" => "localhost",
            _ => panic!("Invalid ENVIRONMENT env"),
        };
        // Url containing the effective domain name
        let rp_origin = Url::parse(match ENVIRONMENT() {
            "DEV" => "http://localhost:3000",
            _ => panic!("Invalid ENVIRONMENT env"),
        })
        .expect("Cant parse rp_origin");
        let builder = WebauthnBuilder::new(rp_id, &rp_origin)
            .expect("Invalid configuration")
            .rp_name("Nightly Connect Cloud");

        // Consume the builder and create our webauthn instance.
        let webauthn = Arc::new(builder.build().expect("Invalid configuration"));

        Self {
            db: db_arc,
            geo_location: geo_loc_requester,
            sessions_cache,
            mailer,
            dns_resolver,
            webauthn,
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
