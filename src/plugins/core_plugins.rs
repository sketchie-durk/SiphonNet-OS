use async_trait::async_trait;
use bytes::{Bytes, BytesMut, BufMut};
use anyhow::Result;
use tracing::{info, error};
use tokio::fs::OpenOptions;
use tokio::io::AsyncWriteExt;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct LogPlugin {
    pub log_file: Arc<Mutex<tokio::fs::File>>,
}

impl LogPlugin {
    pub async fn new(path: &str) -> Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .await?;
        Ok(Self {
            log_file: Arc::new(Mutex::new(file)),
        })
    }
}

#[async_trait]
impl crate::core::engine::Plugin for LogPlugin {
    fn name(&self) -> &str {
        \"LogPlugin\"
    }

    async fn on_request(&self, data: &mut Bytes) -> Result<bool> {
        let mut file = self.log_file.lock().await;
        let log_line = format!(\"[REQ] Len: {} | Data: {:?}\\n\", data.len(), data.as_ref());
        file.write_all(log_line.as_bytes()).await?;
        Ok(true)
    }

    async fn on_response(&self, data: &mut Bytes) -> Result<bool> {
        let mut file = self.log_file.lock().await;
        let log_line = format!(\"[RES] Len: {} | Data: {:?}\\n\", data.len(), data.as_ref());
        file.write_all(log_line.as_bytes()).await?;
        Ok(true)
    }
}

pub struct ObfuscationPlugin {
    pub key: u8,
}

impl ObfuscationPlugin {
    pub fn new(key: u8) -> Self {
        Self { key }
    }
}

#[async_trait]
impl crate::core::engine::Plugin for ObfuscationPlugin {
    fn name(&self) -> &str {
        \"ObfuscationPlugin\"
    }

    async fn on_request(&self, _data: &mut Bytes) -> Result<bool> {
        Ok(true)
    }

    async fn on_response(&self, data: &mut Bytes) -> Result<bool> {
        let mut new_data = BytesMut::with_capacity(data.len());
        for &b in data.iter() {
            new_data.put_u8(b ^ self.key);
        }
        *data = new_data.freeze();
        info!(\"[Obfuscation] Payload XOR-masked with key 0x{:02X}\", self.key);
        Ok(true)
    }
}
