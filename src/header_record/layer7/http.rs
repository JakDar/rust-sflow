#[derive(Default, Debug, Clone, PartialEq)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub host: Option<String>,
}

impl HttpRequest {
    pub fn new(method: String, path: String, host: Option<String>) -> HttpRequest {
        HttpRequest { method, path, host }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct HttpResponse {
    pub status_code: i32,
}

impl HttpResponse {
    pub fn new(status_code: i32) -> HttpResponse {
        HttpResponse { status_code }
    }
}