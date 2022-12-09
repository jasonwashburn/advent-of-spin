use anyhow::Result;
use serde_json::json;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

/// A simple Spin HTTP component.
#[http_component]
fn advent_of_spin_1(req: Request) -> Result<Response> {
    println!("{:?}", req.headers());
    let resp = json!({
        "message": "Hello, world!"
    });
    Ok(http::Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(Some(resp.to_string().into()))?)
}
