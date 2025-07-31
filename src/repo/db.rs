use dotenvy::dotenv;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;
use std::time::Duration;

pub async fn init_postgres_pool(db_url: &str) -> PgPool {
    let pool: PgPool = PgPoolOptions::new()
        .max_connections(10)
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Some(Duration::from_secs(100)))
        .max_lifetime(Some(Duration::from_secs(100)))
        .connect(db_url)
        .await
        .expect("数据库连接失败");
    pool
}

pub fn load_db_url() -> String {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("获取数据库URL失败");
    db_url
}
