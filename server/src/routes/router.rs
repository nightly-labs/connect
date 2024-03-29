use super::cloud_router::cloud_router;
use crate::{
    cloud_state::CloudState,
    handle_error::handle_error,
    http::relay::{
        connect_session::connect_session, drop_sessions::drop_sessions,
        get_pending_request::get_pending_request, get_pending_requests::get_pending_requests,
        get_session_info::get_session_info, get_sessions::get_sessions,
        get_wallets_metadata::get_wallets_metadata, resolve_request::resolve_request,
    },
    ip_geolocation::GeolocationRequester,
    middlewares::cloud_middleware::cloud_middleware,
    sesssion_cleaner::start_cleaning_sessions,
    state::ServerState,
    structs::http_endpoints::HttpEndpoint,
    utils::{get_cors, get_wallets_metadata_vec},
    ws::{
        app_handler::handler::on_new_app_connection,
        client_handler::handler::on_new_client_connection,
    },
};
use axum::{
    error_handling::HandleErrorLayer,
    middleware,
    routing::{get, post},
    Router,
};
use database::db::Db;
use std::{sync::Arc, time::Duration};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

pub async fn get_router(only_relay_service: bool) -> Router {
    let (db, geo_loc_requester, cloud_state) = if only_relay_service {
        (None, None, None)
    } else {
        let db_arc = Arc::new(Db::connect_to_the_pool().await);
        let geo_loc_requester = Arc::new(GeolocationRequester::new().await);

        let cloud_state = CloudState {
            db: db_arc.clone(),
            geo_location: geo_loc_requester.clone(),
        };

        (
            Some(db_arc),
            Some(geo_loc_requester),
            Some(Arc::new(cloud_state)),
        )
    };

    let state = ServerState {
        sessions: Default::default(),
        client_to_sessions: Default::default(),
        client_to_sockets: Default::default(),
        wallets_metadata: Arc::new(get_wallets_metadata_vec()),
        session_to_app_map: Default::default(),
        cloud_state: cloud_state,
        db,
        geo_location: geo_loc_requester,
    };

    // Start cleaning outdated sessions
    start_cleaning_sessions(
        &state.sessions,
        &state.client_to_sessions,
        &state.session_to_app_map,
    );
    let cors = get_cors();

    let router = if only_relay_service {
        Router::new()
    } else {
        Router::new()
            .nest(
                "/cloud",
                cloud_router(state.clone()).route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    cloud_middleware,
                )),
            )
            .with_state(state.clone())
    };

    return router
        .route("/client", get(on_new_client_connection))
        .route("/app", get(on_new_app_connection))
        .route(
            &HttpEndpoint::GetSessionInfo.to_string(),
            post(get_session_info),
        )
        .route(
            &HttpEndpoint::ConnectSession.to_string(),
            post(connect_session),
        )
        .route(&HttpEndpoint::GetSessions.to_string(), post(get_sessions))
        .route(&HttpEndpoint::DropSessions.to_string(), post(drop_sessions))
        .route(
            &HttpEndpoint::GetPendingRequests.to_string(),
            post(get_pending_requests),
        )
        .route(
            &HttpEndpoint::GetPendingRequest.to_string(),
            post(get_pending_request),
        )
        .route(
            &HttpEndpoint::ResolveRequest.to_string(),
            post(resolve_request),
        )
        .route(
            &HttpEndpoint::GetWalletsMetadata.to_string(),
            get(get_wallets_metadata),
        )
        .with_state(state)
        .layer(TraceLayer::new_for_http())
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(handle_error))
                .timeout(Duration::from_secs(10)),
        )
        .layer(cors);
}
