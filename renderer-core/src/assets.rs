pub mod materials;
pub mod models;
pub mod textures;

use std::ops::Range;

pub trait HttpClient: Clone + Send + Sync + 'static {
    #[cfg(feature = "wasm")]
    type Future: std::future::Future<Output = anyhow::Result<Vec<u8>>>;
    #[cfg(not(feature = "wasm"))]
    type Future: std::future::Future<Output = anyhow::Result<Vec<u8>>> + Send;

    fn fetch_bytes(&self, url: &url::Url, range: Option<Range<usize>>) -> Self::Future;
}
