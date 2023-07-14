use serde::{Deserialize, Serialize};
use worker::*;

#[derive(Serialize, Deserialize)]
#[serde()]
struct APIResult {
    pub diff_ms: Option<i128>,
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
async fn main(_req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    let unix_ms = worker::Date::now().as_millis();
    let unix = unix_ms / 1000;

    Response::from_json(&APIResponse {
        status: APIStatus::Success,
        result: APIResult {
            diff_ms: None,
            diff_s: None,
            unix_ms,
            unix,
        },
    })
}
