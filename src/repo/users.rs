use crate::models::users::User;
use crate::services::users::login::{AuthData, TelegramUser};
use sqlx::{Error, PgPool};

#[async_trait::async_trait]
pub trait UserRepoTrait {
    async fn upsert_by_telegram_id(
        &self,
        auth_data: &AuthData<TelegramUser>,
    ) -> Result<User, Error>;
}
pub struct UserRepo<'a> {
    pool: &'a PgPool,
}
impl<'a> UserRepo<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }
}
#[async_trait::async_trait]
impl<'a> UserRepoTrait for UserRepo<'a> {
    async fn upsert_by_telegram_id(
        &self,
        auth_data: &AuthData<TelegramUser>,
    ) -> Result<User, Error> {
        let user = sqlx::query_as!(
User,
r#"
        INSERT INTO users (nickname, avatar_url, telegram_id, status, code, ip_address, os, create_time, last_time, solana_wallet, from_type)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        ON CONFLICT (telegram_id) DO UPDATE
            SET nickname = EXCLUDED.nickname,
                last_time = EXCLUDED.last_time
        RETURNING nickname, avatar_url, telegram_id, status, code, ip_address, os, create_time, last_time, solana_wallet, from_type
        "#,
auth_data.user.username,
auth_data.user.photo_url,
auth_data.user.id,
0,
"unknow".to_string(),
auth_data.ip_address,
auth_data.os,
chrono::Utc::now().naive_utc(),
chrono::Utc::now().naive_utc(),
None::< String >,
"Telegram".to_string()
)
            .fetch_one(&*self.pool)
            .await?;

        Ok(user)
    }
}
