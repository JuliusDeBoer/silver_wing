#[macro_use]
extern crate rocket;

use chrono::Local;
use cuid2::cuid;
use rocket::serde::json::Json;
use shared::{Entry, HealthStatus, SimpleEntry};
use std::sync::Mutex;

static QUEUE: Mutex<Vec<Entry>> = Mutex::new(Vec::new());
static HEALTH: Mutex<HealthStatus> = Mutex::new(HealthStatus {
    healthy: true,
    last_push: None,
    last_clean: None,
});

#[get("/health")]
fn health() -> Json<HealthStatus> {
    let health = HEALTH.lock().expect("Could not get lock");
    Json(health.clone())
}

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
              |  | (   |            | /
              )  |  \\  `.___________|/
              `--'   `--'"
}

#[post("/push", data = "<entry>")]
fn push(entry: Json<SimpleEntry>) {
    let queue_binding = &QUEUE;
    let health_binding = &HEALTH;
    let mut queue = queue_binding.lock().expect("Could not get lock");
    let mut health = health_binding.lock().expect("Could not get lock");

    queue.push(Entry {
        id: cuid(),
        title: entry.title.clone(),
        body: entry.body.clone(),
        date: Local::now(),
    });
    health.last_clean = Some(Local::now());
}

#[get("/entries")]
fn get_entries() -> Json<Vec<shared::Entry>> {
    let queue = QUEUE.lock().expect("Could not get lock");
    Json(queue.clone())
}

#[put("/clear", data = "<entries>")]
fn clear(entries: Json<Vec<String>>) {
    let queue_binding = &QUEUE;
    let health_binding = &HEALTH;
    let mut queue = queue_binding.lock().expect("Could not get lock");
    let mut health = health_binding.lock().expect("Could not get lock");
    queue.retain(|entry| !entries.contains(&entry.id));
    health.last_clean = Some(Local::now());
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![health, push, get_entries, clear, kitty])
}
