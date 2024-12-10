use rocket::http::{ContentType, Status};
use reqwest::Client;
use rocket::State;

use crate::util;

#[get("/blob/<did>/<cid>")]
pub async fn get_blob(client: &State<Client>, did: &str, cid: &str) -> Result<(ContentType, Vec<u8>), Status> {
    match util::get_pds(client, did).await {
        Ok(endpoint) => {
            let blob = match util::get_blob(client, &endpoint, did, cid).await {
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

pub fn routes() -> Vec<rocket::Route> {
    routes![get_blob]
}