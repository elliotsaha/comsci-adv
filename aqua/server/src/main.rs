#[macro_use]
extern crate rocket;
use rocket::serde::{json::Json, Serialize};
use serde_json::json;

#[derive(Serialize)]
struct Person {
    name: String,
    age: u8,
}

#[get("/")]
fn index() -> Json<Person> {
    let person = Person {
        name: String::from("Hello World"),
        age: 12,
    };
    Json(person)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
