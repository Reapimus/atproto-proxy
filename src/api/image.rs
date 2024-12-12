use rocket::http::{Accept, ContentType, Status};
use image::imageops::{resize, FilterType};
#[cfg(feature = "blob_cache")]
use foyer::HybridCache;
use image::ImageReader;
use std::path::PathBuf;
use std::str::FromStr;
use std::io::Cursor;
use reqwest::Client;
use rocket::State;

use crate::params::{ImageType, ProxyParameters};
use crate::config::Config;
use crate::BlobIdentifier;
use crate::util;

async fn get_image_inner(
    client: &State<Client>,
    #[cfg(feature = "blob_cache")]
    cache: &State<HybridCache<BlobIdentifier, Vec<u8>>>,
    config: &State<Config>,
    accepts: &Accept,
    did: &str,
    cid: PathBuf
) -> Result<(ContentType, Vec<u8>), Status> {
    let cid = cid.to_str().unwrap();
    let (cid, parameters) = cid.split_once('@').unwrap_or((cid, ""));
    match util::get_pds(client, did).await {
        Ok(endpoint) => {
            let mut blob = match util::get_blob(
                config,
                client,
                #[cfg(feature = "blob_cache")] cache,
                &endpoint,
                &BlobIdentifier::new(did.to_owned(), cid.to_owned())
            ).await {
                Ok(blob) => blob,
                Err(_) => {
                    return Err(Status::NotFound)
                },
            };
            let mut params: ProxyParameters<ImageType> = match parameters {
                "" => ProxyParameters::default(),
                _ => ProxyParameters::from_str(parameters).map_err(|_| Status::BadRequest)?
            };
            println!("@INFO: Proxy parameters: {:?}", params);
            let mut content_type = util::extract_content_type(&blob);
            if !content_type.to_string().starts_with("image/") {
                return Err(Status::BadRequest)
            }

            if params.file_type == ImageType::Best {
                for t in accepts.iter() {
                    let it = ImageType::from(t);
                    match it {
                        ImageType::Best => params.file_type = ImageType::JPEG,
                        _ => {
                            params.file_type = it;
                            break;
                        }
                    }
                }
            }

            if content_type != params.file_type.into() {
                let reader = ImageReader::new(std::io::Cursor::new(blob)).with_guessed_format().map_err(|_| Status::BadRequest)?;
                content_type = params.file_type.into();
                let mut bytes: Vec<u8> = Vec::new();
                let img = reader.decode().map_err(|_| Status::BadRequest)?;
                img.write_to(
                    &mut Cursor::new(&mut bytes),
                    params.file_type.try_into().map_err(|_| Status::BadRequest)?
                ).map_err(|_| Status::BadRequest)?;
                blob = bytes;
            }

            if let Some(resolution) = params.resolution {
                let reader = ImageReader::new(std::io::Cursor::new(blob)).with_guessed_format().map_err(|_| Status::BadRequest)?;
                let img = reader.decode().map_err(|_| Status::BadRequest)?;
                let buffer = resize(&img, resolution.0, resolution.1, FilterType::Gaussian);
                let mut bytes: Vec<u8> = Vec::new();
                buffer.write_to(
                    &mut Cursor::new(&mut bytes),
                    params.file_type.try_into().map_err(|_| Status::BadRequest)?
                ).map_err(|_| Status::BadRequest)?;
                blob = bytes;
            }

            Ok((content_type, blob))
        },
        Err(_) => {
            return Err(Status::NotFound)
        },
    }
}

#[cfg(feature = "blob_cache")]
#[get("/img/<did>/<cid..>")]
pub async fn get_image(
    client: &State<Client>,
    cache: &State<HybridCache<BlobIdentifier, Vec<u8>>>,
    config: &State<Config>,
    accepts: &Accept,
    did: &str,
    cid: PathBuf
) -> Result<(ContentType, Vec<u8>), Status> {
    return get_image_inner(client, cache, config, accepts, did, cid).await
}

#[cfg(not(feature = "blob_cache"))]
#[get("/img/<did>/<cid..>")]
pub async fn get_image(
    client: &State<Client>,
    config: &State<Config>,
    accepts: &Accept,
    did: &str,
    cid: PathBuf
) -> Result<(ContentType, Vec<u8>), Status> {
    return get_image_inner(client, config, accepts, did, cid).await
}

pub fn routes() -> Vec<rocket::Route> {
    routes![get_image]
}