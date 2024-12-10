use foyer::{HybridCache, HybridCacheBuilder, Engine, DirectFsDeviceOptions};
use serde::{Deserialize, Serialize};

use crate::BlobIdentifier;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub cache: CacheConfig,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CacheConfig {
    pub capacity: usize,
    pub shards: Option<usize>,
    pub disk_location: String,
}

impl CacheConfig {
    pub async fn build(&self) -> HybridCache<BlobIdentifier, Vec<u8>> {
        match self.shards {
            Some(shards) => {
                HybridCacheBuilder::new()
                    .memory(self.capacity)
                    .with_shards(shards)
                    .storage(Engine::Large)
                    .with_device_options(DirectFsDeviceOptions::new(self.disk_location.clone()))
                    .build()
                    .await
                    .unwrap()
            }
            None => {
                HybridCacheBuilder::new()
                    .memory(self.capacity)
                    .storage(Engine::Large)
                    .with_device_options(DirectFsDeviceOptions::new(self.disk_location.clone()))
                    .build()
                    .await
                    .unwrap()
            }
        }
    }
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            capacity: 1024,
            shards: None,
            disk_location: "/data/atproto-proxy".to_owned(),
        }
    }
}