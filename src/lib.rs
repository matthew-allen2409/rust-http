use std::collections::BTreeMap;

pub mod handler;
pub mod request;
pub mod response;
pub mod router;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
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

impl HttpMethod {
    pub fn from(str: &str) -> Option<HttpMethod> {
        match str {
            "GET" => Some(HttpMethod::GET),
            "HEAD" => Some(HttpMethod::HEAD),
            "POST" => Some(HttpMethod::POST),
            "PUT" => Some(HttpMethod::PUT),
            "DELETE" => Some(HttpMethod::GET),
            "CONNECT" => Some(HttpMethod::GET),
            "OPTION" => Some(HttpMethod::GET),
            "TRACE" => Some(HttpMethod::GET),
            "PATCH" => Some(HttpMethod::GET),
            _ => None,
        }
    }
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

pub type Headers = BTreeMap<Box<str>, Box<str>>;
