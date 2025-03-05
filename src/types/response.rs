use crate::types::HttpRequest;

#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub request: HttpRequest,
    pub status_code: u16,
    pub body: Vec<u8>,
}

impl HttpResponse {
    pub fn new(request: HttpRequest, status_code: u16, body: Vec<u8>) -> Self {
        Self {
            request,
            status_code,
            body,
        }
    }
}
