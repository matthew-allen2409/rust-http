use crate::Headers;
use crate::HttpMethod;

#[derive(Debug)]
pub struct Request {
    pub request_line: RequestLine,
    pub headers: Headers,
    pub body: Option<Vec<u8>>,
}

impl Request {
    pub fn new(request_line: RequestLine, headers: Headers, body: Option<Vec<u8>>) -> Request {
        Request {
            request_line,
            headers,
            body,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct RequestLine {
    pub method: HttpMethod,
    pub target: Box<str>,
    pub version: Box<str>,
}

impl RequestLine {
    pub fn from_string(string: &str) -> Option<RequestLine> {
        let mut parts = string.split_whitespace();
        let method = HttpMethod::from(parts.next()?)?;
        let target = Box::from(parts.next()?);
        let version = Box::from(parts.next()?);

        Some(RequestLine::new(method, target,version))
    }

    pub fn new(method: HttpMethod, target: Box<str>, version: Box<str>) -> RequestLine {
        RequestLine {
            method,
            target,
            version
        }
    }
}

#[cfg(test)]
mod tests;
