pub mod handler;
pub mod request;
pub mod response;
pub mod router;

#[derive(Debug)]
pub enum HttpMethod {
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTION,
    TRACE,
    PATCH,
}

#[derive(Debug)]
pub struct Header {
    pub name: String, // Should be case-insensitive
    pub value: String, // Should this be String?
}
