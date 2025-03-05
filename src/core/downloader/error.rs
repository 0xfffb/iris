use thiserror::Error;

#[derive(Error, Debug)]
pub enum HttpDownloaderError {
    #[error("Request error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("Io error: {0}")]
    IoError(#[from] std::io::Error),
}
