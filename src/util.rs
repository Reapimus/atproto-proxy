use rocket::http::ContentType;
use cached::proc_macro::once;
use foyer::HybridCache;
use reqwest::Client;
use anyhow::Result;

use crate::config::Config;
use crate::did_doc::DidDocument;
use crate::BlobIdentifier;

const PLC_DIRECTORY: &str = "https://plc.directory";

pub fn extract_content_type(bytes: &[u8]) -> ContentType {
    let mime = tree_magic_mini::from_u8(bytes);
    ContentType::parse_flexible(mime).unwrap_or(ContentType::Binary)
}

pub async fn get_blob(config: &Config, client: &Client, cache: &HybridCache<BlobIdentifier, Vec<u8>>, endpoint: &str, id: &BlobIdentifier) -> Result<Vec<u8>> {
    if let Some(blob) = cache.get(id).await? {
        println!("@INFO: Found cache entry for blob '{}/{}'", id.did, id.cid);
        return Ok(blob.value().to_owned());
    }
    println!("@INFO: Cache miss for blob '{}/{}'", id.did, id.cid);
    let core_config = &config.core;
    let url = format!("{endpoint}/xrpc/com.atproto.sync.getBlob?did={}&cid={}", id.did, id.cid);
    let blob = client.get(url).send().await?.bytes().await?;
    if core_config.max_blob_size > 0 {
        println!("@INFO: Checking blob size for blob '{}/{}': {}", id.did, id.cid, blob.len());
        if blob.len() > core_config.max_blob_size {
            anyhow::bail!("Blob size exceeds maximum allowed size");
        }
    }
    cache.insert(id.clone(), blob.clone().to_vec());
    Ok(blob.to_vec())
}

#[once(time = 300, result = true, sync_writes = true)]
pub async fn get_pds(client: &Client, did: &str) -> Result<String> {
    let did = did.to_owned();
    let sections = did.split(':').collect::<Vec<&str>>();
    let (_, r#type, id) = (sections[0], sections[1], sections[2]);

    match r#type {
        "plc" => {
            let url = format!("{PLC_DIRECTORY}/{}", did);
            let doc = client.get(url).send().await?.json::<DidDocument>().await?;
            if let Some(endpoint) = doc.get_pds_endpoint() {
                Ok(endpoint)
            } else {
                anyhow::bail!("No PDS endpoint found for DID: {}", did)
            }
        },
        "web" => {
            let url = format!("https://{id}/.well-known/did.json");
            let doc = client.get(url).send().await?.json::<DidDocument>().await?;
            if let Some(endpoint) = doc.get_pds_endpoint() {
                Ok(endpoint)
            } else {
                anyhow::bail!("No PDS endpoint found for DID: {}", did)
            }
        },
        _ => {
            anyhow::bail!("Unsupported DID type: {}", r#type)
        }
    }
}