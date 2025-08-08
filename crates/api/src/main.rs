#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;
use shared::HealthStatus;

#[get("/health")]
fn health() -> Json<HealthStatus> {
    Json(HealthStatus { healthy: true })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![health])
}
