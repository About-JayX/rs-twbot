use axum::Router;
use tower_http::trace::TraceLayer;
pub mod users;
use crate::state::AppState;

pub fn register_router() -> Router {
    Router::new().nest("/api", users::register_router()).layer(TraceLayer::new_for_http()).with_state(AppState::new().clone())
}