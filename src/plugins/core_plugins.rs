use async_trait::async_trait;
use bytes::{Bytes, BytesMut};
use anyhow::Result;
use tracing::info;

pub struct LogPlugin {
    pub log_file: String,
}

#[async_trait]
impl crate::core::engine::Plugin for LogPlugin {
    fn name(&self) -> &str {
        "LogPlugin"
    }

    async fn on_request(&self, data: &mut Bytes) -> Result<bool> {
        info!("[LOG] Request captured. Length: {}", data.len());
        // In a real system, this would asynchronously write to a rotating file
        Ok(true)
    }

    async fn on_response(&self, data: &mut Bytes) -> Result<bool> {
        info!("[LOG] Response captured. Length: {}", data.len());
        Ok(true)
    }
}

pub struct CompressionPlugin;

#[async_trait]
impl crate::core::engine::Plugin for CompressionPlugin {
    fn name(&self) -> &str {
        "CompressionPlugin"
    }

    async fn on_request(&self, _data: &mut Bytes) -> Result<bool> {
        Ok(true)
    }

    async fn on_response(&self, data: &mut Bytes) -> Result<bool> {
        // Mock compression: replace first byte with a compression marker if it's a large payload
        if data.len() > 1024 {
            let mut new_data = BytesMut::from(&data[..]);
            new_data[0] = 0xCC; // Compression Marker
            *data = new_data.freeze();
            info!("[Compression] Payload compressed.");
        }
        Ok(true)
    }
}
