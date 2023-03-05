# Time server CLI

This is a simple command-line interface to the time server.

> **Warning:** This is a work in progress.

## Installation

```sh
cargo install clock-check
```

## Usage

```sh
Usage: clock-check [OPTIONS]

Options:
  -s, --server <SERVER>     The URL of the time server [default: http://localhost:8000/time]
  -b, --bare                Only output the server unix server time
      --seconds             Use seconds instead of milliseconds
  -t, --timeout <TIMEOUT>   A timeout in milliseconds [default: 1000]
  -l, --latency-in-account  Try to account for network latency. This is not very accurate and should be considered experimental
  -u, --use-ntp             Use NTP to get the time instead of a time server (experimental). This will use time.cloudflare.com:123 by default, but you can specify a different server with the --server flag
  -h, --help                Print help
  -V, --version             Print version
```