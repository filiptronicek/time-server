#[macro_use] extern crate rocket;
use rocket::serde::{Serialize, json::Json};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct ResultDefault {
    unix_ms: u64,
    unix: u64
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct ResultWithDifference {
    diff_ms: u64,
    diff_s: u64,
    unix_ms: u64,
    unix: u64
}

fn get_unix_times() -> (u64, u64) {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    println!("{:?}", since_the_epoch);

    let unix = since_the_epoch.as_secs();
    let unix_ms = unix * 1000 +
    since_the_epoch.subsec_nanos() as u64 / 1_000_000;
    (unix_ms, unix);
    return (unix_ms, unix);
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct Response {
    status: Status,
    result: ResultDefault
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct ResponseWithDifference {
    status: Status,
    result: ResultWithDifference
}

#[derive(Serialize)]
enum Status {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "error")]
    Error
}

#[get("/time")]
fn time() -> Json<Response> {
    let (unix_ms, unix) = get_unix_times();
    let result = ResultDefault { unix_ms, unix };
    let response = Response { status: Status::Success, result };
    Json(response)
}

fn round_to_nearest(number: u64, denominator: u64) -> u64 {
    (number + (denominator / 2)) / denominator * denominator
}

#[get("/time?<ts>")]
fn time_query(ts: u64) -> Json<ResponseWithDifference> {
    let (unix_ms, unix) = get_unix_times();

    let diff_ms = unix_ms - ts;
    let diff_s = round_to_nearest(diff_ms, 1000) / 1000;

    let result = ResultWithDifference { unix_ms, unix, diff_s, diff_ms };
    let response = ResponseWithDifference { status: Status::Success, result };
    Json(response)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![time, time_query])
}
