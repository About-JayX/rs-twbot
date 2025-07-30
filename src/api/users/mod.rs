pub mod handlers;
use axum::{Router};
use crate::state::AppState;

pub fn register_router()->Router<AppState>{
    Router::new().nest("/user",handlers::handlers_users())
}