#[macro_use]
extern crate rocket;

use cuid2::cuid;
use rocket::serde::json::Json;
use shared::Entry;
use std::sync::Mutex;

static QUEUE: Mutex<Vec<Entry>> = Mutex::new(Vec::new());

#[get("/kitty")]
fn kitty() -> &'static str {
    "                                     ,
              ,-.       _,---._ __  / \\
             /  )    .-'       `./ /   \\
            (  (   ,'            `/    /|
             \\  `-\"             \\'\\   / |
              `.              ,  \\ \\ /  |
               /`.          ,'-`----Y   |
              (            ;        |   '
              |  ,-.    ,-'         |  /
              |  | (   |        hjw | /
              )  |  \\  `.___________|/
              `--'   `--'"
}

#[post("/push", data = "<entry>")]
fn push(entry: &str) {
    let binding = &QUEUE;
    let mut queue = binding.lock().expect("Could not get lock");
    queue.push(Entry {
        id: cuid(),
        body: entry.into(),
    });
}

#[get("/entries")]
fn get_entries() -> Json<Vec<shared::Entry>> {
    let queue = QUEUE.lock().expect("Could not get lock");
    Json(queue.clone())
}

#[put("/clear", data = "<entries>")]
fn clear(entries: Json<Vec<String>>) {
    let binding = &QUEUE;
    let mut queue = binding.lock().expect("Could not get lock");
    queue.retain(|entry| !entries.contains(&entry.id));
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![push, get_entries, clear, kitty])
}
