use clap::Parser;
use std::error::Error;
use utils::get_unix_times;
use utils::ResponseWithDifference;

extern crate clap;
extern crate serde;
extern crate utils;
extern crate serde_json;

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

    /// A timeout in milliseconds
    #[arg(short, long, default_value = "1000")]
    timeout: u64,

    /// Try to account for network latency. This is not very accurate and should be considered experimental
    #[arg(short, long, default_value = "false")]
    latency_in_account: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let (client_unix_ms, _) = get_unix_times();
    let url = if !args.bare {
        format!("{}?ts={}", args.server, client_unix_ms)
    } else {
        args.server
    };

    let server_response = reqwest::blocking::get(&url)?;

    let resp = match server_response.error_for_status() {
        Ok(resp) => resp,
        Err(err) => {
            println!("Error: {}", err);
            return Ok(());
        }
    };

    let (client_end_unix_ms, _) = get_unix_times();

    let client_diff_ms = client_end_unix_ms - client_unix_ms;
    if client_diff_ms > args.timeout {
        println!("Request took too long ({}ms)", client_diff_ms);
        return Ok(());
    }

    let resp = match resp.json::<ResponseWithDifference>() {
        Ok(resp) => resp,
        Err(err) => {
            println!("Server response parsing Error: {}", err);
            return Ok(());
        }
    };

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

    let route_to_server_ms = (resp.result.unix_ms - client_unix_ms) / 2;

    let unix_difference = match resp.result.diff_ms {
        Some(diff) => diff,
        None => {
            println!("Server did not return a difference");
            return Ok(());
        }
    };

    let unix_difference = if args.latency_in_account {
        unix_difference - route_to_server_ms as i128
    } else {
        unix_difference
    };

    let ahead_or_behind = if unix_difference > 0 {
        "behind"
    } else {
        "ahead"
    };

    let unix_difference = if args.seconds {
        unix_difference as f32 / 1000f32
    } else {
        unix_difference as f32
    };

    if unix_difference == 0f32 {
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
