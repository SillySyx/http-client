mod request;
mod response;
mod methods;

#[cfg(feature = "azure")]
pub mod azure;

pub use {
    request::HttpRequest,
    response::HttpResponse,
    methods::HttpMethods,
};