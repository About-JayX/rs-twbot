use crate::api::response::Response;
use crate::state::AppState;
use axum::extract::State;
use axum::response::IntoResponse;
use sqlx::Row;

pub async fn login_handler(State(state): State<AppState>) -> impl IntoResponse {
    let pool = &state.pg_pool;

    let row = sqlx::query("SELECT id FROM  users").fetch_all(pool).await;
    match row {
        Ok(row) => {
            let ids: Vec<i64> = row.iter().filter_map(|r| r.try_get("id").ok()).collect();

            Response::<String>::success(ids[0].to_string())
        }
        Err(e) => Response::error("Login Error".to_string()),
    }
}
