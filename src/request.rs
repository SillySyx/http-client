use std::{error::Error, str::FromStr};

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

use crate::{HttpMethods, HttpResponse};

#[derive(Clone)]
pub struct HttpRequest {
    pub url: Option<String>,
    pub method: Option<HttpMethods>,
    pub headers: HeaderMap,
    pub body: Option<Vec<u8>>,
}

impl HttpRequest {
    pub fn new() -> Self {
        Self {
            method: None,
            url: None,
            headers: HeaderMap::new(),
            body: None,
        }
    }

    pub fn with_url(mut self, url: String) -> HttpRequest {
        self.url = Some(url);
        self
    }

    pub fn with_header(mut self, name: String, value: String) -> HttpRequest {
        let name = match HeaderName::from_str(&name) {
            Ok(value) => value,
            Err(_) => return self,
        };

        let value = match HeaderValue::from_str(&value) {
            Ok(value) => value,
            Err(_) => return self,
        };

        self.headers.insert(name, value);
        self
    }

    pub fn with_body<T>(mut self, url: String) -> HttpRequest {
        self.url = Some(url);
        self
    }

    pub async fn get(mut self) -> Result<HttpResponse, Box<dyn Error>> {
        self.method = Some(HttpMethods::Get);
        send(self).await
    }

    pub async fn post(mut self) -> Result<HttpResponse, Box<dyn Error>> {
        self.method = Some(HttpMethods::Post);
        send(self).await
    }

    pub async fn put(mut self) -> Result<HttpResponse, Box<dyn Error>> {
        self.method = Some(HttpMethods::Put);
        send(self).await
    }

    pub async fn patch(mut self) -> Result<HttpResponse, Box<dyn Error>> {
        self.method = Some(HttpMethods::Patch);
        send(self).await
    }

    pub async fn delete(mut self) -> Result<HttpResponse, Box<dyn Error>> {
        self.method = Some(HttpMethods::Delete);
        send(self).await
    }
}

async fn send(request: HttpRequest) -> Result<HttpResponse, Box<dyn Error>> {
    let method = match request.method {
        Some(HttpMethods::Delete) => reqwest::Method::DELETE,
        Some(HttpMethods::Get) => reqwest::Method::GET,
        Some(HttpMethods::Patch) => reqwest::Method::PATCH,
        Some(HttpMethods::Post) => reqwest::Method::POST,
        Some(HttpMethods::Put) => reqwest::Method::PUT,
        None => return Err(Box::from("No method specified")),
    };

    let url = match request.url {
        Some(url) => url,
        None => return Err(Box::from("No url specified")),
    };
    
    let body = match request.body {
        Some(body) => body,
        None => vec![],
    };

    let builder = reqwest::Client::new()
        .request(method, url)
        .headers(request.headers)
        .body(body);

    let response = builder.send().await?;

    let status_code = response.status().as_u16();
    let bytes = response.bytes().await?;
    let bytes = bytes.to_vec();

    Ok(HttpResponse {
        body: bytes,
        status_code,
    })
}