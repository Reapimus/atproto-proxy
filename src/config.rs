#[cfg(feature = "blob_cache")]
use foyer::{HybridCache, HybridCacheBuilder, Engine, DirectFsDeviceOptions};
use serde::{Deserialize, Serialize};
#[cfg(feature = "signing")]
use secp256k1::Keypair;

#[cfg(feature = "blob_cache")]
use crate::BlobIdentifier;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub core: CoreConfig,
    #[cfg(feature = "blob_cache")]
    pub cache: CacheConfig,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoreConfig {
    pub max_blob_size: usize,
    #[cfg(feature = "signing")]
    pub signing_key: Option<Keypair>,
}

impl Default for CoreConfig {
    fn default() -> Self {
        Self {
            max_blob_size: 3_000_000,
            #[cfg(feature = "signing")]
            signing_key: None,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg(feature = "blob_cache")]
pub struct CacheConfig {
    pub capacity: usize,
    pub shards: Option<usize>,
    pub disk_location: String,
}

#[cfg(feature = "blob_cache")]
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

#[cfg(feature = "blob_cache")]
impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            capacity: 1024,
            shards: None,
            disk_location: "/data/atproto-proxy".to_owned(),
        }
    }
}