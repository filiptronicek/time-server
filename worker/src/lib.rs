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
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();

    router
        .get("/stamp/:time", |_req, ctx| {
            let unix_ms = worker::Date::now().as_millis();
            let unix = unix_ms / 1000;

            let client_time = match ctx.param("time") {
                Some(time) => match time.parse::<u64>() {
                    Ok(parsed_time) => parsed_time,
                    Err(_) => return Response::error("Bad Request", 400),
                },
                None => return Response::error("Bad Request", 400),
            };

            let client_time_s = client_time / 1000;
            let diff_ms = unix_ms as i128 - client_time as i128;
            let diff_s = unix as i128 - client_time_s as i128;
            let result = APIResult {
                diff_ms: Some(diff_ms),
                diff_s: Some(diff_s),
                unix_ms,
                unix,
            };
            let response = APIResponse {
                status: APIStatus::Success,
                result,
            };
            Response::from_json(&response)
        })
        .get("/stamp", |_req, _ctx| {
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
        })
        .run(req, env)
        .await
}
