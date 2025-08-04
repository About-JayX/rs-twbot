use crate::api::response::Response;
use crate::models::users::User;
use crate::repo::users::{UserRepo, UserRepoTrait};
use crate::state::AppState;
use axum::Json;
use axum::extract::{ConnectInfo, State};
use axum::http::{HeaderMap, header};
use axum::response::IntoResponse;
use core::str;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Debug, Serialize, Deserialize)]
pub struct TelegramUser {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub language_code: String,
    pub allows_write_to_pm: bool,
    pub photo_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthData<T> {
    pub user: T,
    pub chat_instance: String,
    pub chat_type: String,
    pub auth_date: u64,
    pub signature: String,
    pub hash: String,
    pub ip_address: Option<String>,
    pub os: Option<String>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    pub init_data: String,
}

pub async fn handler_login(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, Response<()>> {
    let LoginRequest { init_data } = payload;

    let parse_data: AuthData<String> = serde_urlencoded::from_str(&init_data)
        .map_err(|_| Response::error("failed parse init data".to_string()))?;

    let user: TelegramUser = serde_json::from_str(&parse_data.user)
        .map_err(|_| Response::error("failed parse telegram user".to_string()))?;

    let auth_data: AuthData<TelegramUser> = AuthData {
        user,
        chat_instance: parse_data.chat_instance,
        chat_type: parse_data.chat_type,
        auth_date: parse_data.auth_date,
        signature: parse_data.signature,
        hash: parse_data.hash,
        ip_address: Some(addr.ip().to_string()),
        os: Some(
            headers
                .get(header::USER_AGENT)
                .and_then(|v| v.to_str().ok())
                .unwrap_or("unknown")
                .to_string(),
        ),
    };
    let user_repo = UserRepo::new(&state.pg_pool);

    let user: User = user_repo.upsert_by_telegram_id(&auth_data)
        .await
        .map_err(|_| Response::error("failed to query user".to_string()))?;

    Ok(Response::success(user))
}