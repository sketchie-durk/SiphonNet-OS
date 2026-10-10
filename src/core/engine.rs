use async_trait::async_trait;
use bytes::Bytes;
use anyhow::Result;

#[async_trait]
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    async fn on_request(&self, data: &mut Bytes) -> Result<bool>;
    async fn on_response(&self, data: &mut Bytes) -> Result<bool>;
}

pub struct Engine {
    plugins: Vec<Box<dyn Plugin>>,
}

impl Engine {
    pub fn new() -> Self {
        Self { plugins: Vec::new() }
    }

    pub fn add_plugin<P: Plugin + 'static>(&mut self, plugin: P) {
        self.plugins.push(Box::new(plugin));
    }

    pub async fn process_request(&self, data: &mut Bytes) -> Result<bool> {
        for plugin in &self.plugins {
            if !plugin.on_request(data).await? {
                return Ok(false);
            }
        }
        Ok(true)
    }

    pub async fn process_response(&self, data: &mut Bytes) -> Result<bool> {
        for plugin in &self.plugins {
            if !plugin.on_response(data).await? {
                return Ok(false);
            }
        }
        Ok(true)
    }
}
