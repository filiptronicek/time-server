use clap::Parser;
use nippy::protocol::Packet;
use std::error::Error;
use utils::get_unix_times;
use utils::Response;

extern crate clap;
extern crate serde;
extern crate serde_json;
extern crate time_server_utils as utils;

use anyhow::Result;

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

    /// Use NTP to get the time instead of a time server (experimental).
    /// This will use time.cloudflare.com:123 by default, but you can specify a different server with the --server flag
    #[arg(long, default_value = "false")]
    use_ntp: bool,

    /// Print the help message for the markdown version of this program
    #[arg(long, hide = true)]
    markdown_help: bool,
}

async fn get_unix_ntp_time(pool_ntp: &str) -> Result<Packet, Box<dyn Error>> {
    let response = nippy::request(pool_ntp).await?;
    Ok(response)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    if args.markdown_help {
        clap_markdown::print_help_markdown::<Args>();
        return Ok(());
    }

    let (client_unix_ms, _) = get_unix_times();
    let mut unix_difference: f64;

    if args.use_ntp {
        let ntp_server = if args.server == "http://localhost:8000/time" {
            "time.cloudflare.com:123"
        } else {
            &args.server
        };
        let ntp_result = get_unix_ntp_time(ntp_server).await.unwrap();
        let (client_receive_time, _) = get_unix_times();

        let ntp_receive_time = ntp_result.receive_timestamp;
        let ntp_transmit_time = ntp_result.transmit_timestamp;
        let ntp_receive_instant = nippy::Instant::from(ntp_receive_time);
        let ntp_transmit_instant = nippy::Instant::from(ntp_transmit_time);
        let ntp_receive_ms = ntp_receive_instant.secs() as f64 * 1000f64
            + ntp_receive_instant.subsec_nanos() as f64 / 1_000_000f64;
        let ntp_transmit_ms = ntp_transmit_instant.secs() as f64 * 1000f64
            + ntp_transmit_instant.subsec_nanos() as f64 / 1_000_000f64;

        if args.bare {
            println!(
                "{}",
                if args.seconds {
                    ntp_receive_instant.secs() as f64
                } else {
                    ntp_receive_ms
                }
            );
            return Ok(());
        }

        // Print T1, T2, T3, T4
        println!(
            "
            T1: {}ms
            T2: {}ms
            T3: {}ms
            T4: {}ms",
            client_unix_ms, ntp_receive_ms, ntp_transmit_ms, client_receive_time
        );

        unix_difference = 
            (ntp_receive_ms - client_unix_ms as f64) +
            (ntp_transmit_ms - client_receive_time as f64);

        if args.latency_in_account {
            unix_difference = unix_difference / 2f64;
        }
        
    } else {
        let url = if !args.bare {
            format!("{}?ts={}", args.server, client_unix_ms)
        } else {
            args.server
        };

        let server_response = reqwest::get(&url).await?;

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

        let resp = match resp.json::<Response>().await {
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

        unix_difference = match resp.result.diff_ms {
            Some(diff) => diff as f64,
            None => {
                println!("Server did not return a difference");
                return Ok(());
            }
        };

        unix_difference = if args.latency_in_account {
            unix_difference - route_to_server_ms as f64
        } else {
            unix_difference
        };
    }

    let ahead_or_behind = if unix_difference > 0f64 {
        "behind"
    } else {
        "ahead"
    };

    let unix_difference = if args.seconds {
        unix_difference / 1000f64
    } else {
        unix_difference
    };

    if unix_difference == 0f64 {
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
