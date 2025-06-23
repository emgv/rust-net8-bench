use axum::http::StatusCode;
use axum::response::{Json as AxumJson, IntoResponse};
use serde::Serialize;

type AxumResponse = axum::response::Response<axum::body::Body>;

pub mod cars;
pub mod prime;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct WebApiError {
    pub message: String,
}

impl WebApiError {
    pub fn new(message: &str) -> Self {
        Self {
            message: String::from(message)
        }
    }
}

fn fast_serialize_json<T: Serialize>(value: &T) -> Result<bytes::Bytes, serde_json::Error> {
    let vec = serde_json::to_vec(value)?;
    Ok(bytes::Bytes::from(vec))
}

fn make_response_from_object<T: Serialize>(status: StatusCode, value: &T) -> Result<AxumResponse, Box<dyn std::error::Error>> {
    let body = fast_serialize_json(value)?;
    Ok(axum::response::Response::builder()
        .status(status)
        .header(axum::http::header::CONTENT_TYPE, "application/json; charset=utf-8")
        .header("Server", "Axummmmmmmmmmm")
        .body(axum::body::Body::from(body))?)
}

fn make_response_from_byte(status: StatusCode, body: bytes::Bytes) -> Result<AxumResponse, Box<dyn std::error::Error>> {
    Ok(axum::response::Response::builder()
        .status(status)
        .header(axum::http::header::CONTENT_TYPE, "application/json")
        .body(axum::body::Body::from(body))?)
}

fn make_web_api_error_response(status: StatusCode, message: &str) -> AxumResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, AxumJson(WebApiError::new(message)))
        .into_response()
}