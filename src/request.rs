use std::{str::FromStr, collections::HashMap};

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

use crate::HttpMethod;

#[derive(Clone)]
pub struct HttpRequest {
    pub url: String,
    pub method: HttpMethod,
    pub headers: HeaderMap,
    pub query_params: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl HttpRequest {
    pub fn new() -> Self {
        Self {
            method: HttpMethod::Get,
            url: String::new(),
            headers: HeaderMap::new(),
            query_params: HashMap::new(),
            body: vec![],
        }
    }

    pub fn with_url<URL: Into<String>>(mut self, url: URL) -> HttpRequest {
        self.url = url.into();
        self
    }

    pub fn with_header(mut self, name: &str, value: &str) -> HttpRequest {
        let name = match HeaderName::from_str(name) {
            Ok(value) => value,
            Err(_) => return self,
        };

        let value = match HeaderValue::from_str(value) {
            Ok(value) => value,
            Err(_) => return self,
        };

        self.headers.insert(name, value);
        self
    }

    pub fn with_query_param<VALUE: Into<String>>(mut self, name: VALUE, value: VALUE) -> HttpRequest {
        self.query_params.insert(name.into(), value.into());
        self
    }

    pub fn with_body<BODY: Into<Vec<u8>>>(mut self, body: BODY) -> HttpRequest {
        self.body = body.into();
        self
    }

    pub fn with_method(mut self, method: HttpMethod) -> HttpRequest {
        self.method = method;
        self
    }
}