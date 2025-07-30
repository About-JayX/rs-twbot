use axum::{serve};
use tokio::net::TcpListener;

pub mod api;
pub mod state;

use crate::api::register_router;


#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = register_router();
    let listener = TcpListener::bind("127.0.0.1:8989").await?;
    serve(listener, app).await?;
    Ok(())
}
