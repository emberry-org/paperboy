#[macro_use] extern crate rocket;

mod github;
mod update;

use std::sync::Mutex;
use github::get_release;
use once_cell::sync::Lazy;
use rocket::serde::json::Json;

/// Cache for the latest update JSON.
pub(crate) static CACHED_UPDATE: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(r#"{"loaded":false}"#.to_string()));

#[get("/updates")]
fn updates<'a>() -> Json<String> {
    Json::from(CACHED_UPDATE.lock().unwrap().to_owned())
}

#[launch]
fn rocket() -> _ {
    get_release();
    rocket::build().mount("/", routes![updates])
}
