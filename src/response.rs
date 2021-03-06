use serde::Deserialize;

use crate::HttpError;

pub struct HttpResponse {
    pub status_code: u16,
    pub body: Vec<u8>,
}

impl HttpResponse {
    pub fn successful_status_code(&self) -> bool {
        self.status_code >= 200 && self.status_code < 300
    }

    pub fn body_as<'a, T>(&'a self) -> Result<T, HttpError> where T: Deserialize<'a> {
        let body = match serde_json::from_slice::<T>(&self.body) {
            Ok(value) => value,
            Err(error) => return Err(HttpError::FailedToDeserializeResponseBody(format!("{:?}", error))),
        };

        Ok(body)
    }
}