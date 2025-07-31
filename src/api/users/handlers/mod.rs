use crate::state::AppState;
use axum::Router;
use axum::routing::get;

pub mod login;

pub fn handlers_users() -> Router<AppState> {
    Router::new().route("/login", get(login::login_handler))
}
