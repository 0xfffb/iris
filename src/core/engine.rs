use std::{collections::HashMap, sync::Arc};

use crate::{
    core::downloader::{HttpDownloader, HttpDownloaderError},
    types::{HttpMethod, HttpRequest},
};

pub struct Engine {
    downloader: Arc<HttpDownloader>,
}

impl Engine {
    pub fn new(downloader: HttpDownloader) -> Self {
        Self {
            downloader: Arc::new(downloader),
        }
    }

    pub async fn start(&self) {
        let semaphore = Arc::new(tokio::sync::Semaphore::new(1));

        loop {
            let _downloader = Arc::clone(&self.downloader);
            let sem = Arc::clone(&semaphore);
            tokio::spawn(async move {
                let _permit = sem.acquire().await.unwrap();
                let request = HttpRequest::new(
                    "https://m.media-amazon.com/images/I/71u8AWoBOwL._AC_SY550_.jpg".to_string(),
                    HttpMethod::GET,
                    HashMap::new(),
                    None,
                );
                match _downloader.fetch(&request).await {
                    Ok(response) => {
                        println!("{:?}", response.body.len());
                    }
                    Err(e) => {
                        Self::handle_error(e);
                    }
                };
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            });
        }
    }

    fn handle_error(e: HttpDownloaderError) {
        match e {
            HttpDownloaderError::RequestError(e) => {
                println!("Request error: {:?}", e);
            }
            HttpDownloaderError::IoError(e) => {
                println!("Io error: {:?}", e);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_engine() {
        let engine = Engine::new(HttpDownloader::new());
        engine.start().await;
    }
}
