mod utils;

use wasm_bindgen::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Serialize, Deserialize};
use serde_json::json;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
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

#[wasm_bindgen]
pub fn greet() {
    alert("Hello");
    let (client_unix_ms, client_unix) = get_unix_times();

    // Fetch the time from the server
    let mut opts = web_sys::RequestInit::new();
    opts.method("GET");

    let request = web_sys::Request::new_with_str_and_init(
        "http://localhost:8000/time",
        &opts,
    ).unwrap();

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await.unwrap();

    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    assert_eq!(resp.status(), 200);

    let text = JsFuture::from(resp.text().unwrap()).await.unwrap();
    let text: String = text.as_string().unwrap().into();

    let json: Value = serde_json::from_str(&text).unwrap();
    let server_unix_ms = json["result"]["unix_ms"].as_u64().unwrap();
    let server_unix = json["result"]["unix"].as_u64().unwrap();

    let diff_ms = server_unix_ms - client_unix_ms;
    let diff_s = server_unix - client_unix;

    return diff_ms;
}
