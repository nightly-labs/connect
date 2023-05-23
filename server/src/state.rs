use std::sync::Arc;

use axum_macros::FromRef;
use dashmap::DashMap;

use crate::structs::session::Session;

#[derive(Clone, FromRef)]
pub struct ServerState {
    pub sessions: Arc<DashMap<String, Session>>,
}
