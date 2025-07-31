use crate::state::AppState;
use axum::Router;
use axum::routing::post;

use crate::services::users::login::handler_login;
pub fn handlers_users() -> Router<AppState> {
    Router::new().route("/login", post(handler_login))
}

pub fn register_users_router() -> Router<AppState> {
    Router::new().nest("/user", handlers_users())
}
