mod compressor;
mod service;

#[macro_use]
extern crate rocket;

use rocket::http::ContentType;
use rocket::response::status::Custom;
use rocket::Data;
use rocket_raw_response::RawResponse;

use service::ImageCompressService;

#[get("/")]
fn index() -> &'static str {
    "Hello"
}

#[post("/", data = "<data>")]
async fn conpress_img(
    content_type: &ContentType,
    data: Data<'_>,
) -> Result<RawResponse, Custom<String>> {
    let srv = ImageCompressService::new();
    let result = srv.compress(content_type, data).await;

    match result {
        Ok(result) => Ok(result),
        Err(e) => Err(e),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/compress_img", routes![conpress_img])
}
