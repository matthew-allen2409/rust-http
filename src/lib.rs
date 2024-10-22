use std::collections::BTreeMap;

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

#[derive(Debug, PartialEq)]
pub struct Header {
    pub name: Box<str>,
    pub value: Box<str>,
}

impl Header {
    pub fn new(name: Box<str>, value: Box<str>) -> Self {
        Self { name, value }
    }

    pub fn to_string(&self) -> String {
        format!("{}: {}\r\n", self.name, self.value)
    }
}

pub type Handler = fn(Vec<String>, Headers) -> response::Response;
pub type Headers = BTreeMap<Box<str>, Box<str>>;
