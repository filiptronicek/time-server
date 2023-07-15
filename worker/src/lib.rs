use serde::{Deserialize, Serialize};
use worker::*;

#[derive(Serialize, Deserialize)]
#[serde()]
struct APIResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diff_ms: Option<i128>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diff_s: Option<i128>,
    pub unix_ms: u64,
    pub unix: u64,
}

#[derive(Serialize, Deserialize)]
#[serde()]
struct APIResponse {
    pub status: APIStatus,
    pub result: APIResult,
}

#[derive(Serialize, Deserialize)]
enum APIStatus {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "error")]
    _Error,
}

#[event(fetch)]
async fn main(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    let unix_ms = worker::Date::now().as_millis();
    let unix = unix_ms / 1000;

    let parsed_url = match req.url() {
        Ok(url) => url,
        Err(e) => {
            println!("Error parsing URL: {:?}", e);
            return Err(e);
        }
    };

    let ts_value = parsed_url
        .query_pairs()
        .find(|(key, _)| key == "ts")
        .map(|(_, value)| value.into_owned());

    let (diff_ms, diff_s) = match ts_value {
        Some(time) => match time.parse::<u64>() {
            Ok(client_time) => {
                let client_time_s = client_time / 1000;
                let diff_ms = unix_ms as i128 - client_time as i128;
                let diff_s = unix as i128 - client_time_s as i128;
                (Some(diff_ms), Some(diff_s))
            }
            Err(_) => return Response::error("Bad Request", 400),
        },
        None => (None, None),
    };

    let result = APIResult {
        diff_ms,
        diff_s,
        unix_ms,
        unix,
    };
    let response = APIResponse {
        status: APIStatus::Success,
        result,
    };
    Response::from_json(&response)
}
