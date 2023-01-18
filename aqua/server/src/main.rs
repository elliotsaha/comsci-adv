use utils::{ApiResponse, ApiStatuses, ResponseStructure};
pub mod utils;

#[macro_use]
extern crate rocket;
use rocket::serde::{json::Json, Serialize};

#[derive(Serialize)]
struct Person {
    name: String,
    age: u8,
}

#[get("/")]
fn index() -> Json<ResponseStructure<String>> {
    ApiResponse::error(String::from("Hello"))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
