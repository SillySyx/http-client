use std::collections::HashMap;

use crate::{HttpRequest, HttpMethod, HttpResponse, HttpError};

pub async fn send_request(request: HttpRequest) -> Result<HttpResponse, HttpError> {
    let method = match request.method {
        HttpMethod::Delete => reqwest::Method::DELETE,
        HttpMethod::Get => reqwest::Method::GET,
        HttpMethod::Patch => reqwest::Method::PATCH,
        HttpMethod::Post => reqwest::Method::POST,
        HttpMethod::Put => reqwest::Method::PUT,
    };

    if request.url.is_empty() {
        return Err(HttpError::NoUrlSpecified);
    }

    let mut url = request.url;

    if let Some(query_params) = build_query_string(&request.query_params) {
        url = format!("{}?{}", url, query_params);
    }
    
    let builder = reqwest::Client::new()
        .request(method, url)
        .headers(request.headers)
        .body(request.body);

    let response = match builder.send().await {
        Ok(value) => value,
        Err(error) => return Err(HttpError::FailedToSend(format!("{:?}", error)))
    };

    let status_code = response.status().as_u16();

    let bytes = match response.bytes().await {
        Ok(value) => value,
        Err(_) => return Err(HttpError::FailedToReadResponseBytes),
    };

    let bytes = bytes.to_vec();

    Ok(HttpResponse {
        body: bytes,
        status_code,
    })
}

fn build_query_string(query_params: &HashMap<String, String>) -> Option<String> {

    let value: Vec<String> = query_params
        .iter()
        .map(|(key, value)| format!("{key}={value}"))
        .collect();

        let value = value.join("&");

    if value.is_empty() {
        return None;
    }

    Some(value)
}