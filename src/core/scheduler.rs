use std::collections::HashMap;

use crate::types::{HttpMethod, HttpRequest};

pub struct Scheduler {}

impl Scheduler {
    pub fn new() -> Self {
        Self {}
    }

    pub fn next(&self) -> Option<HttpRequest> {
        let request = HttpRequest::new(
            "https://m.media-amazon.com/images/I/71u8AWoBOwL._AC_SY550_.jpg".to_string(),
            HttpMethod::GET,
            HashMap::new(),
            None,
        );
        Some(request)
    }
}
