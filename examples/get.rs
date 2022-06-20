use std::collections::HashMap;
use tokio::main;

use http_client::{HttpRequest, HttpMethod, send_request};

#[main]
async fn main() {
    let request = HttpRequest::new()
        .with_url("https://httpbin.org/ip")
        .with_method(HttpMethod::Get);

    let response = send_request(request)
        .await
        .expect("Failed to send request");

    if !response.successful_status_code() {
        panic!("AAAAA");
    }

    let body = response.body_as::<HashMap<String, String>>().expect("Failed to parse body");

    println!("{:#?}", body);
}