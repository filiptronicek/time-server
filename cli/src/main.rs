use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

extern crate serde;
extern crate serde_json;

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(crate = "serde")]
struct ResultWithDifference {
    diff_ms: i128,
}

#[derive(Deserialize)]
#[serde(crate = "serde")]
struct ResponseWithDifference {
    result: ResultWithDifference
}

fn get_unix_times() -> (u64, u64) {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let unix = since_the_epoch.as_secs();
    let unix_ms = unix * 1000 +
    since_the_epoch.subsec_nanos() as u64 / 1_000_000;
    (unix_ms, unix);
    return (unix_ms, unix);
}

fn main() -> Result<(), Box<dyn Error>> {

    let (unix_ms, _unix) = get_unix_times();
    let url = format!("http://localhost:8000/time?ts={}", unix_ms);

    let resp = reqwest::blocking::get(&url)?;
    let resp: ResponseWithDifference = resp.json()?;

    let unix_difference = resp.result.diff_ms;
    let ahead_or_behind = if unix_difference > 0 { "behind" } else { "ahead" };

    if unix_difference == 0 {
        println!("Your clock is in sync!");
        return Ok(());
    }

    println!("Your clock is {:?}ms {}", unix_difference.abs(), ahead_or_behind);
    Ok(())
}