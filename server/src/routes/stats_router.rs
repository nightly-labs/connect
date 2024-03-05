use crate::state::ServerState;
use axum::Router;

pub fn stats_router(state: ServerState) -> Router<ServerState> {
    Router::new().with_state(state)
}
