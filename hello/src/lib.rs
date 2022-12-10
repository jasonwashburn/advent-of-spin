use std::str::FromStr;

use anyhow::Result;
use serde_json::json;
use serde::{Serialize, Deserialize};
use http::Uri;
use spin_sdk::{
    http::{Request, Response},
    http_component,
};

#[derive(Deserialize, Serialize)]
struct LowercaseRequest {
    value: String,
}

#[derive(Deserialize, Serialize)]
struct LowercaseResponse {
    message: String,
}

/// A simple Spin HTTP component.
#[http_component]
fn advent_of_spin_1(req: Request) -> Result<Response> {
    println!("{:?}", req.headers());
    let mut name: String = dbg!(req.headers().get("spin-path-info").unwrap().to_str().unwrap().to_string());
    if name != "" {

        let url = req.headers().get("spin-full-url").unwrap().to_str().unwrap();
        let uri = Uri::from_str(url).unwrap();
        let domain = uri.host().unwrap().to_string();
        let port = match uri.port() {
            Some(v) => format!(":{}", v.to_string()),
            None => "".to_string(),
        };
        let method = uri.scheme_str().unwrap().to_string();
        let lower_req = LowercaseRequest {
            value: name.to_string().strip_prefix("/").unwrap().to_string(),
        };

        let resp = spin_sdk::outbound_http::send_request(
            http::Request::builder()
                .method("POST")
                .uri(dbg!(format!("https://{}{}/lowercase", domain, port)))
                .body(Some(serde_json::to_string(&lower_req).unwrap().into())).unwrap()
        );
        dbg!(&resp);
        let lowercase_resp: LowercaseResponse = serde_json::from_str(&String::from_utf8(resp.unwrap().body().to_owned().unwrap().to_vec()).unwrap()).unwrap();
        name = lowercase_resp.message.to_owned();
    } else {
        name = "world".to_string();
    };
//    lowercase_resp.headers_mut()
//        .insert("spin-component", "rust-outbound-http".try_into()?);

    let resp = json!({
        "message": format!("Hello, {}!", name)
    });
    Ok(http::Response::builder()
        .status(200)
        .header("Content-Type", "application/json")
        .body(Some(resp.to_string().into()))?)
}
