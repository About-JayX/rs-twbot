use axum::serve;
use tokio::net::TcpListener;

pub mod api;
pub mod application;
pub mod infrastructure;
pub mod state;

use crate::api::register_router;
use crate::infrastructure::db::{init_postgres_pool, load_db_url};
use crate::state::AppState;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // connect db
    let db_url = load_db_url();
    let pool = init_postgres_pool(&db_url).await;

    // start server
    let app = register_router(AppState {
        pg_pool: pool,
        user_service: None,
    });
    let listener = TcpListener::bind("127.0.0.1:8989").await?;
    serve(listener, app).await?;
    Ok(())
}
