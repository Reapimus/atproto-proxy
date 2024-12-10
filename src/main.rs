#[macro_use] extern crate rocket;

use reqwest::ClientBuilder;
use config::Config;
use serde::{Deserialize, Serialize};

#[catch(404)]
fn not_found() -> &'static str {
    "Not found"
}

#[catch(400)]
fn bad_request() -> &'static str {
    "Bad request"
}

#[launch]
async fn rocket() -> _ {
    let rocket = rocket::build();
    let figment = rocket.figment();
    let config: Config = figment.extract().expect("Failed to extract config");
    
    let client = ClientBuilder::new()
        .user_agent(format!("atproto-proxy/{} (https://github.com/Reapimus/atproto-proxy)", env!("CARGO_PKG_VERSION")))
        .build()
        .unwrap();

    let cache = config.cache.build().await;

    rocket
        .manage(client)
        .manage(cache)
        .manage(config)
        .register("/", catchers![not_found, bad_request])
        .mount("/", api::routes())
}

#[derive(Clone, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlobIdentifier {
    pub did: String,
    pub cid: String,
}

impl BlobIdentifier {
    pub fn new(did: String, cid: String) -> Self {
        Self { did, cid }
    }
}

mod did_doc;
mod params;
mod config;
mod util;
mod api;