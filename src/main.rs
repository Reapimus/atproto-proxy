#[macro_use] extern crate rocket;

use reqwest::ClientBuilder;

#[catch(404)]
fn not_found() -> &'static str {
    "Not found"
}

#[catch(400)]
fn bad_request() -> &'static str {
    "Bad request"
}

#[launch]
fn rocket() -> _ {
    let client = ClientBuilder::new()
        .user_agent(format!("atproto-proxy/{} (https://github.com/Reapimus/atproto-proxy)", env!("CARGO_PKG_VERSION")))
        .build()
        .unwrap();

    rocket::build()
        .manage(client)
        .register("/", catchers![not_found, bad_request])
        .mount("/", api::routes())
}

mod did_doc;
mod params;
mod util;
mod api;