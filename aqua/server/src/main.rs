pub mod errors;
pub mod utils;

use errors::{internal_err, not_found};
use utils::{ApiRes, ApiResponse, ApiStatuses};

#[macro_use]
extern crate rocket;
extern crate reqwest;

#[get("/")]
fn index() -> ApiRes<String> {
    ApiResponse::success(String::from("Entry point"))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![not_found, internal_err])
        .mount("/api", routes![index])
}
