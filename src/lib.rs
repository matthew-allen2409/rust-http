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
    pub fn from_string(str: &String) -> Option<Header> {
        let mut parts = str.split(':').map(|part| part.trim());
        let name = Box::from(parts.next()?);
        let value = Box::from(parts.next()?);

        Some(Header::new(name, value))
    }

    pub fn new(name: Box<str>, value: Box<str>) -> Self {
        Self { name, value }
    }

    pub fn to_string(&self) -> String {
        format!("{}: {}\r\n", self.name, self.value)
    }
}

pub type Headers = BTreeMap<Box<str>, Box<str>>;

#[cfg(test)]
mod tests {
    use super::Header;

    #[test]
    fn header_from_string_expect_header() {
        let header_str = String::from("Content-Length: 69420");

        let expected = Header {
            name: Box::from("Content-Length"),
            value: Box::from("69420"),
        };

        let result = Header::from_string(&header_str).unwrap();

        assert_eq!(expected, result);
    }
}
