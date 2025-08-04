use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Serialize;

#[derive(Serialize)]
pub struct Response<T> {
    pub status: bool,
    pub message: Option<String>,
    pub data: Option<T>,
}
impl<T> Response<T> {
    pub fn success(data: T) -> Self {
        Response {
            status: true,
            message: Some("success".to_string()),
            data: Some(data),
        }
    }

    pub fn error(message: String) -> Self {
        Response {
            status: false,
            message: Some(message),
            data: None,
        }
    }
}

impl<T: Serialize> IntoResponse for Response<T> {
    fn into_response(self) -> axum::response::Response {
        let code = if self.status {
            StatusCode::OK
        } else {
            StatusCode::BAD_REQUEST
        };

        (code, Json(self)).into_response()
    }
}
