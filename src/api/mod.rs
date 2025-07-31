use axum::Router;
use tower_http::trace::TraceLayer;
mod response;
pub mod users;

use crate::state::AppState;

pub fn register_router(state: AppState) -> Router {
    Router::new()
        .nest("/api", users::register_users_router())
        .layer(TraceLayer::new_for_http())
        .with_state(state.clone())
}
