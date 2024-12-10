pub fn routes() -> Vec<rocket::Route> {
    let mut list = Vec::new();
    list.append(&mut image::routes());
    list.append(&mut blob::routes());
    list
}

mod blob;
mod image;