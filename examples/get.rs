use std::collections::HashMap;
use tokio::main;

use http_client::HttpRequest;

#[main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = HttpRequest::new()
        .with_url("https://httpbin.org/ip".to_string())
        .get()
        .await?;

    if !response.successful_status_code() {
        panic!("AAAAA");
    }

    let body = response.body_as::<HashMap<String, String>>()?;

    println!("{:#?}", body);
    
    Ok(())
}