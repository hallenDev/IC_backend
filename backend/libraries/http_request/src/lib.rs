mod router;
mod logs_handler;
pub mod images;

use serde::Serialize;
use serde_bytes::ByteBuf;
use types::{HeaderField, HttpResponse};

pub use router::*;
pub use logs_handler::*;

pub fn build_json_response<T: Serialize>(body: &T) -> HttpResponse {
    let bytes = serde_json::to_string(body).unwrap().into_bytes();

    build_response(bytes, "application/json")
}

pub fn build_response(body: Vec<u8>, content_type: impl Into<String>) -> HttpResponse {
    HttpResponse {
        status_code: 200,
        headers: vec![
            HeaderField("Content-Type".to_string(), content_type.into()),
            HeaderField("Content-Length".to_string(), body.len().to_string()),
        ],
        body: ByteBuf::from(body),
    }
}
