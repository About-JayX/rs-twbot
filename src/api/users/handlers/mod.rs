use axum::Router;
use axum::routing::get;
use crate::state::AppState;

pub mod login;


pub fn handlers_users() -> Router<AppState> {
    Router::new().route("/login", get(login::login_handler()))
}