#[macro_use] extern crate rocket;
use rocket::serde::json::Json;

extern crate time_server_utils as utils;
use utils::{Response, Status, Result};
use utils::{get_unix_times, round_to_nearest};

#[get("/time")]
fn time() -> Json<Response> {
    let (unix_ms, unix) = get_unix_times();
    let result = Result { unix_ms, unix, diff_s: None, diff_ms: None };
    let response = Response { status: Status::Success, result };
    Json(response)
}

#[get("/time?<ts>")]
fn time_query(ts: u64) -> Json<Response> {
    let (unix_ms, unix) = utils::get_unix_times();

    let diff_ms = unix_ms as i128 - ts as i128;
    let diff_s = round_to_nearest(diff_ms, 1000) / 1000;

    let result = Result { unix_ms, unix, diff_s: Some(diff_s), diff_ms: Some(diff_ms) };
    let response = Response { status: Status::Success, result };
    Json(response)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![time, time_query])
}
