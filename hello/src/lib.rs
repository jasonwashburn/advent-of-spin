use anyhow::Result;
use serde_json::json;
use serde::{Serialize, Deserialize};
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

#[derive(Deserialize, Serialize)]
struct LowercaseRequest {
    value: String,
}


/// A simple Spin HTTP component.
#[http_component]
fn advent_of_spin_1(req: Request) -> Result<Response> {
    println!("{:?}", req.headers());
    let name = dbg!(req.headers().get("spin-path-info").unwrap().to_str().unwrap());

    let lower_req = LowercaseRequest {
        value: name.to_string().strip_prefix("/").unwrap().to_string(),
    };

    let mut lowercase_resp = spin_sdk::outbound_http::send_request(
        http::Request::builder()
            .method("POST")
            .uri("http://127.0.0.1:3000/lowercase")
            .body(Some(serde_json::to_string(&lower_req).unwrap().into())).unwrap()
    );
//    lowercase_resp.headers_mut()
//        .insert("spin-component", "rust-outbound-http".try_into()?);
    println!("{:?}", lowercase_resp);

    let resp = json!({
        "message": "Hello, world!"
    });
    Ok(http::Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(Some(resp.to_string().into()))?)
}
