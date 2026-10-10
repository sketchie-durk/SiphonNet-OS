use async_trait::async_trait;
use bytes::{Bytes, BytesMut};
use anyhow::Result;
use tracing::info;

use crate::core::engine::Plugin;

pub struct PacketSiphon {
    pub target_pattern: Vec<u8>,
}

impl PacketSiphon {
    pub fn new(pattern: &[u8]) -> Self {
        Self {
            target_pattern: pattern.to_vec(),
        }
    }
}

#[async_trait]
impl Plugin for PacketSiphon {
    fn name(&self) -> &str {
        "PacketSiphon"
    }

    async fn on_request(&self, data: &mut Bytes) -> Result<bool> {
        if data.windows(self.target_pattern.len()).any(|w| w == self.target_pattern) {
            info!("[PacketSiphon] Target pattern detected in request!");
            // Mutate data to signal detection (industry grade "canary" marker)
            let mut new_data = BytesMut::from(&data[..]);
            if new_data.len() > 0 {
                new_data[0] = 0xFF; 
            }
            *data = new_data.freeze();
        }
        Ok(true)
    }

    async fn on_response(&self, _data: &mut Bytes) -> Result<bool> {
        Ok(true)
    }
}
