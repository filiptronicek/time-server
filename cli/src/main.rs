use clap::Parser;
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

extern crate clap;
extern crate serde;
extern crate serde_json;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(crate = "serde")]
struct ResultWithDifference {
    diff_ms: Option<i128>,
    diff_s: Option<i128>,
    unix_ms: u64,
    unix: u64,
}

/// A simple program to get the time from a time server
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The URL of the time server
    #[arg(short, long, default_value = "http://localhost:8000/time")]
    server: String,

    /// Only output the server unix server time
    #[arg(short, long, default_value = "false")]
    bare: bool,

    /// Use seconds instead of milliseconds
    #[arg(long, default_value = "false")]
    seconds: bool,

    /// A timeout in miliseconds
    #[arg(short, long, default_value = "1000")]
    timeout: u64,
}

#[derive(Deserialize)]
#[serde(crate = "serde")]
struct ResponseWithDifference {
    result: ResultWithDifference,
}

fn get_unix_times() -> (u64, u64) {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    let unix = since_the_epoch.as_secs();
    let unix_ms = unix * 1000 + since_the_epoch.subsec_nanos() as u64 / 1_000_000;
    (unix_ms, unix);
    return (unix_ms, unix);
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let (client_unix_ms, _) = get_unix_times();
    let url = if !args.bare {
        format!("{}?ts={}", args.server, client_unix_ms)
    } else {
        args.server
    };

    let resp = reqwest::blocking::get(&url)?;
    let (client_end_unix_ms, _) = get_unix_times();

    let client_diff_ms = client_end_unix_ms - client_unix_ms;
    if client_diff_ms > args.timeout {
        println!("Request took too long ({}ms)", client_diff_ms);
        return Ok(());
    }

    let resp: ResponseWithDifference = resp.json()?;

    if args.bare {
        println!(
            "{}",
            if args.seconds {
                resp.result.unix
            } else {
                resp.result.unix_ms
            }
        );
        return Ok(());
    }

    let unix_difference = if args.seconds {
        resp.result.diff_s.unwrap()
    } else {
        resp.result.diff_ms.unwrap()
    };
    let ahead_or_behind = if unix_difference > 0 {
        "behind"
    } else {
        "ahead"
    };

    if unix_difference == 0 {
        println!("Your clock is in sync!");
        return Ok(());
    }

    println!(
        "Your clock is {:?}{}s {}",
        unix_difference.abs(),
        if args.seconds { "" } else { "m" },
        ahead_or_behind
    );
    Ok(())
}
