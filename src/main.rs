mod core;
mod types;

#[tokio::main]
async fn main() {
    let engine = core::engine::Engine::new(core::downloader::http::HttpDownloader::new());
    engine.start().await;
}
