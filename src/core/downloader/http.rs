use crate::core::downloader::error::HttpDownloaderError;
use crate::types::HttpMethod;
use crate::types::HttpRequest;
use crate::types::HttpResponse;
use reqwest::Client;

#[derive(Clone, Debug)]
pub struct HttpDownloader {
    client: Client,
}

impl HttpDownloader {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn fetch(&self, request: &HttpRequest) -> Result<HttpResponse, HttpDownloaderError> {
        match request.method {
            HttpMethod::GET => self._fetch_get(request).await,
            HttpMethod::POST => {
                todo!()
            }
            HttpMethod::PUT => {
                todo!()
            }
            HttpMethod::DELETE => {
                todo!()
            }
        }
    }

    async fn _fetch_get(&self, request: &HttpRequest) -> Result<HttpResponse, HttpDownloaderError> {
        let response = self.client.get(request.url.clone()).send().await?;
        let status_code = response.status().as_u16();
        let bytes = response.bytes().await?;

        Ok(HttpResponse::new(
            request.clone(),
            status_code,
            bytes.to_vec(),
        ))
    }
}
