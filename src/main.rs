#[macro_use] extern crate rocket;
use rocket::serde::{Serialize, json::Json};
use serde::Deserialize;
use std::time::{SystemTime, UNIX_EPOCH};

use rocket::request::FromParam;

#[derive(Debug)]
struct TsParam {
    ts: Option<String>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct ResultDefault {
    unix_ms: u64,
    unix: u64
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Response {
    status: String,
    result: ResultDefault
}

#[get("/time")]
fn time() -> Json<Response> {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    println!("{:?}", since_the_epoch);

    let unix = since_the_epoch.as_secs();
    let unix_ms = unix * 1000 +
    since_the_epoch.subsec_nanos() as u64 / 1_000_000;
    let result = ResultDefault { unix_ms, unix };
    let response = Response { status: "ok".to_string(), result };
    Json(response)
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct TimeQuery {
    client: Option<u64>,
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![time])
}
