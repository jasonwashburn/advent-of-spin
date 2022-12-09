use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

#[derive(Serialize, Deserialize)]
struct RequestBody {
    value: String,
}

#[derive(Serialize, Deserialize)]
struct ResponseBody {
    message: String,
}


/// A simple Spin HTTP component.
#[http_component]
fn lowercase(req: Request) -> Result<Response> {
    println!("{:?}", req.headers());
    req.body().to_owned().unwrap();
    let req_bytes = req.body(); 
    let req_str = String::from_utf8(req_bytes.to_owned().unwrap().to_vec()).unwrap();
    let body: RequestBody = serde_json::from_str(&req_str).unwrap();
    let resp_value: String = body.value.to_owned().to_lowercase();
    let resp = ResponseBody {
        message: resp_value,
    };
    Ok(http::Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(Some(serde_json::to_string(&resp).unwrap().into()))?)
}
