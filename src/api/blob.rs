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
    cid: &str,
    sig: Option<String>
) -> Result<(ContentType, Vec<u8>), Status> {
    #[cfg(feature = "signing")]
    if sig.is_some() {
        let data = format!("blob/{did}/{cid}");
        let bytes = data.as_bytes();
        util::validate_signature(&config, &sig.unwrap(), bytes).map_err(|_| Status::Unauthorized)?
    } else {
        return Err(Status::Unauthorized)
    }
    #[cfg(not(feature = "signing"))]
    if sig.is_some() {
        return Err(Status::BadRequest)
    }
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
#[get("/blob/<did>/<cid>?<sig>")]
pub async fn get_blob(
    client: &State<Client>,
    cache: &State<HybridCache<BlobIdentifier, Vec<u8>>>,
    config: &State<Config>,
    did: &str,
    cid: &str,
    sig: Option<String>
) -> Result<(ContentType, Vec<u8>), Status> {
    get_blob_inner(client, cache, config, did, cid, sig).await
}

#[cfg(not(feature = "blob_cache"))]
#[get("/blob/<did>/<cid>?<sig>")]
pub async fn get_blob(
    client: &State<Client>,
    config: &State<Config>,
    did: &str,
    cid: &str,
    sig: Option<String>
) -> Result<(ContentType, Vec<u8>), Status> {
    get_blob_inner(client, config, did, cid, sig).await
}

pub fn routes() -> Vec<rocket::Route> {
    routes![get_blob]
}