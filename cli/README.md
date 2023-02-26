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
  -s, --server <SERVER>  The URL of the time server [default: http://localhost:8000/time]
  -b, --bare             Only output the server unix server time
      --seconds          Use seconds instead of milliseconds
  -h, --help             Print help
  -V, --version          Print version
```