use rocket::http::ContentType;
use cached::proc_macro::once;
use reqwest::Client;
use anyhow::Result;

use crate::did_doc::DidDocument;

const PLC_DIRECTORY: &str = "https://plc.directory";

pub fn extract_content_type(bytes: &[u8]) -> ContentType {
    let mime = tree_magic_mini::from_u8(bytes);
    ContentType::parse_flexible(mime).unwrap_or(ContentType::Binary)
}

pub async fn get_blob(client: &Client, endpoint: &str, did: &str, cid: &str) -> Result<Vec<u8>> {
    let url = format!("{endpoint}/xrpc/com.atproto.sync.getBlob?did={did}&cid={cid}");
    let blob = client.get(url).send().await?.bytes().await?;
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