use super::scheduler::Scheduler;
use crate::core::downloader::{HttpDownloader, HttpDownloaderError};
use std::sync::Arc;

pub struct Engine {
    downloader: Arc<HttpDownloader>,
    scheduler: Arc<Scheduler>,
}

impl Engine {
    pub fn new(downloader: HttpDownloader, scheduler: Scheduler) -> Self {
        Self {
            downloader: Arc::new(downloader),
            scheduler: Arc::new(scheduler),
        }
    }

    pub async fn start(&self) {
        let semaphore = Arc::new(tokio::sync::Semaphore::new(1));
        loop {
            let _downloader = Arc::clone(&self.downloader);
            let _scheduler = Arc::clone(&self.scheduler);
            let sem = Arc::clone(&semaphore);
            tokio::spawn(async move {
                let _permit = sem.acquire().await.unwrap();
                if let Some(request) = _scheduler.next() {
                    match _downloader.fetch(&request).await {
                        Ok(response) => {
                            println!("{:?}", response.body.len());
                        }
                        Err(e) => {
                            Self::handle_error(e);
                        }
                    }
                }
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
        let engine = Engine::new(HttpDownloader::new(), Scheduler::new());
        engine.start().await;
    }
}
