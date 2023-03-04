use std::time::{SystemTime, UNIX_EPOCH};
use rocket::serde::{Serialize, Deserialize};

extern crate serde;
extern crate serde_json;

pub fn get_unix_times() -> (u64, u64) {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let unix = since_the_epoch.as_secs();
    let unix_ms = unix * 1000 + since_the_epoch.subsec_nanos() as u64 / 1_000_000;
    (unix_ms, unix);
    return (unix_ms, unix);
}

pub fn round_to_nearest(number: i128, denominator: i128) -> i128 {
    (number + (denominator / 2)) / denominator * denominator
}

#[derive(Serialize, Deserialize)]
#[serde()]
pub struct ResultDefault {
    pub unix_ms: u64,
    pub unix: u64
}

#[derive(Serialize, Deserialize)]
#[serde()]
pub struct ResultWithDifference {
    pub diff_ms: Option<i128>,
    pub diff_s: Option<i128>,
    pub unix_ms: u64,
    pub unix: u64
}

#[derive(Serialize, Deserialize)]
#[serde()]
pub struct Response {
    pub status: Status,
    pub result: ResultDefault
}

#[derive(Serialize, Deserialize)]
#[serde()]
pub struct ResponseWithDifference {
    pub status: Status,
    pub result: ResultWithDifference
}

#[derive(Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "error")]
    _Error
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unix() {
        let (unix_ms, unix) = get_unix_times();
        println!("unix_ms: {}", unix_ms);
        println!("unix: {}", unix);
        assert!(unix_ms > 0);
        assert!(unix > 0);
        assert!(unix_ms > unix);
    }

    #[test]
    fn round() {
        assert_eq!(round_to_nearest(1499, 1000), 1000);
        assert_eq!(round_to_nearest(1500, 1000), 2000);
        assert_eq!(round_to_nearest(1501, 1000), 2000);
        assert_eq!(round_to_nearest(1499, 100), 1500);
        assert_eq!(round_to_nearest(1500, 100), 1500);
        assert_eq!(round_to_nearest(1501, 100), 1500);
    }
}
