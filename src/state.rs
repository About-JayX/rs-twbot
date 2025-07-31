use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct AppState {
    pub user_service: Option<Arc<String>>,
    pub pg_pool: PgPool,
}
impl AppState {
    pub fn new(state: AppState) -> AppState {
        state
    }
}
