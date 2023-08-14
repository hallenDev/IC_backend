use candid::CandidType;
use serde::Deserialize;
use serde_bytes::ByteBuf;

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct HeaderField(pub String, pub String);

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<HeaderField>,
    pub body: ByteBuf,
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct HttpResponse {
    pub status_code: u16,
    pub headers: Vec<HeaderField>,
    pub body: ByteBuf,
}

impl HttpRequest {
    pub fn header(&self, key: &str) -> Option<&String> {
        let key_lower = key.to_lowercase();
        self.headers
            .iter()
            .find(|f| f.0.to_lowercase() == key_lower)
            .map(|f| &f.1)
    }
}

impl HttpResponse {
    pub fn status_code(code: u16) -> HttpResponse {
        HttpResponse {
            status_code: code,
            headers: Vec::new(),
            body: ByteBuf::default(),
        }
    }

    pub fn not_found() -> HttpResponse {
        HttpResponse::status_code(404)
    }
}
