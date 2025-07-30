use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub user_service: Arc<String>,
}
impl AppState {
    pub fn new() -> AppState {
        AppState {
            user_service:Arc::new(String::from("")),
        }
    }
}