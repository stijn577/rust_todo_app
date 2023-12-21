#[cfg(not(test))]
static WASM_DIR: &str = "dist";
#[cfg(test)]
static WASM_DIR: &str = "../dist";

use rocket::{response::content::RawJson, Build, Rocket};
use shared_lib::utils::structs::{TaskBuilder, Test, Undefined};

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
        // .mount("/", rocket::fs::FileServer::from(WASM_DIR))
        .mount("/", routes![hello_world, get_tasks])
}

#[get("/hello")]
fn hello_world() -> &'static str {
    "Hello, everyone!"
}

#[get("/tasks")]
fn get_tasks() -> RawJson<String> {
    // let tasks: SerdeWrapper<Vec<Task>> = SerdeWrapper::new(vec![Task::default(), Task::default()]);
    let mut task1 = TaskBuilder::new();
    task1
        .add_title("task 1")
        .add_description("task 1 testing")
        .add_image("no image path yet")
        .add_type(Test::default())
        .add_chapter("1.2")
        .add_subject("undefined subject lol");

    let mut task2 = TaskBuilder::new();
    task2
        .add_title("task 2")
        .add_description("just testing here really")
        .add_image("no image path yet for task 2")
        .add_type(Undefined);

    let task1 = task1.build();
    let task2 = task2.build();

    let json1 = serde_json::to_value(task1).unwrap();
    let json2 = serde_json::to_value(task2).unwrap();

    let tasks = vec![json1, json2];

    RawJson(serde_json::to_string(&tasks).unwrap())
}
