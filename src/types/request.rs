use std::collections::HashMap;

use crate::types::HttpMethod;

#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub url: String,
    pub method: HttpMethod,
    pub headers: HashMap<String, String>,
    pub proxy: Option<String>,
}

impl HttpRequest {
    pub fn new(
        url: String,
        method: HttpMethod,
        headers: HashMap<String, String>,
        proxy: Option<String>,
    ) -> Self {
        Self {
            url,
            method,
            headers,
            proxy,
        }
    }
}

impl Default for HttpRequest {
    fn default() -> Self {
        Self {
            url: String::new(),
            method: HttpMethod::GET,
            headers: HashMap::new(),
            proxy: None,
        }
    }
}
