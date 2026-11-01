//! HTTP requests and responses, described without being sent.
//!
//! The core has no network access (D-008): it builds the request and parses
//! the response; the terminal (or browser) program is the one sending it.

/// HTTP method.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Method {
    Get,
    Post,
}

/// Request body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Body {
    Empty,
    Json(String),
    Multipart(Vec<Part>),
}

/// Part of a `multipart/form-data` body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Part {
    Text {
        name: String,
        value: String,
    },
    File {
        name: String,
        filename: String,
        mime_type: String,
        bytes: Vec<u8>,
    },
}

/// Request to send to a provider.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpRequest {
    pub method: Method,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: Body,
}

/// Response received from a provider.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpResponse {
    pub status: u16,
    pub body: Vec<u8>,
}

impl HttpResponse {
    /// True for a 2xx status.
    pub fn is_success(&self) -> bool {
        (200..300).contains(&self.status)
    }
}
