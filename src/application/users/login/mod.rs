use crate::state::AppState;
use axum::Json;
use axum::extract::State;
use serde_json::Value;

pub async fn handler_login(State(state): State<AppState>, Json(payload): Json<Value>) {}
