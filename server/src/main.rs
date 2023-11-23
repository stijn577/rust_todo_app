#[cfg(not(test))]
static WASM_DIR: &str = "dist";
#[cfg(test)]
static WASM_DIR: &str = "../dist";

use rocket::{response::content::RawJson, Build, Rocket};
use yew_rocket::utils::structs::Person;

#[macro_use]
extern crate rocket;

#[cfg(test)]
mod tests;

//TODO: add database support to the project
#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        // This binds the yew wasm frontend files, without this the index.html links to .js and .wasm files
        // would not be able to be established.
        .mount("/", rocket::fs::FileServer::from(WASM_DIR))
        .mount("/hello", routes![hello_world, struct_json])
}

#[get("/")]
fn hello_world() -> &'static str {
    "Hello, everyone!"
}

#[get("/<name>/<age>")]
fn struct_json(name: &str, age: u8) -> RawJson<String> {
    let person = Person::new(name, age);
    RawJson(serde_json::to_string(&person).unwrap())
}
