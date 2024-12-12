use rocket::http::{ContentType, Status};
#[cfg(feature = "blob_cache")]
use foyer::HybridCache;
use reqwest::Client;
use rocket::State;

use crate::config::Config;
use crate::BlobIdentifier;
use crate::util;

async fn get_blob_inner(
    client: &State<Client>,
    #[cfg(feature = "blob_cache")]
    cache: &State<HybridCache<BlobIdentifier, Vec<u8>>>,
    config: &State<Config>,
    did: &str,
    cid: &str
) -> Result<(ContentType, Vec<u8>), Status> {
    match util::get_pds(client, did).await {
        Ok(endpoint) => {
            let blob = match util::get_blob(
                config,
                client,
                #[cfg(feature = "blob_cache")]
                cache,
                &endpoint,
                &BlobIdentifier::new(did.to_owned(), cid.to_owned())
            ).await {
                Ok(blob) => blob,
                Err(_) => {
                    return Err(Status::NotFound)
                },
            };
            let content_type = util::extract_content_type(&blob);

            Ok((content_type, blob))
        },
        Err(_) => {
            return Err(Status::NotFound)
        },
    }
}

#[cfg(feature = "blob_cache")]
#[get("/blob/<did>/<cid>")]
pub async fn get_blob(
    client: &State<Client>,
    cache: &State<HybridCache<BlobIdentifier, Vec<u8>>>,
    config: &State<Config>,
    did: &str,
    cid: &str
) -> Result<(ContentType, Vec<u8>), Status> {
    get_blob_inner(client, cache, config, did, cid).await
}

#[cfg(not(feature = "blob_cache"))]
#[get("/blob/<did>/<cid>")]
pub async fn get_blob(
    client: &State<Client>,
    config: &State<Config>,
    did: &str,
    cid: &str
) -> Result<(ContentType, Vec<u8>), Status> {
    get_blob_inner(client, config, did, cid).await
}

pub fn routes() -> Vec<rocket::Route> {
    routes![get_blob]
}