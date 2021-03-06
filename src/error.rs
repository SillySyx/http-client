#[derive(Debug)]
pub enum HttpError {
    NoUrlSpecified,
    FailedToSend(String),
    FailedToReadResponseBytes,
    FailedToDeserializeResponseBody(String),
}