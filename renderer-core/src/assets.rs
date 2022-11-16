pub mod models;
pub mod textures;

use std::ops::Range;

#[cfg(not(feature = "wasm"))]
pub type HttpClientFuture =
    std::pin::Pin<Box<dyn core::future::Future<Output = anyhow::Result<Vec<u8>>> + Send>>;

#[cfg(feature = "wasm")]
pub type HttpClientFuture =
    std::pin::Pin<Box<dyn core::future::Future<Output = anyhow::Result<Vec<u8>>>>>;

pub trait HttpClient: Clone + Send + Sync + 'static {
    fn fetch_bytes(&self, url: &url::Url, range: Option<Range<usize>>) -> HttpClientFuture;
}
