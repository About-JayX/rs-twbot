use serde::{Deserialize, Serialize};

use sqlx::FromRow;
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub nickname: String,
    pub avatar_url: String,
    pub telegram_id: i64,
    pub status: i32,
    pub code: String,
    pub ip_address: String,
    pub os: String,
    pub create_time: chrono::NaiveDateTime,
    pub last_time: chrono::NaiveDateTime,
    pub solana_wallet: Option<String>,
    pub from_type: String,
}
