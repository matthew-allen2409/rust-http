use crate::Headers;
use crate::HttpMethod;
use std::error::Error;

#[derive(Debug)]
pub struct RequestLine {
    pub method: HttpMethod,
    pub target: Box<str>,
    pub version: Box<str>,
}

impl RequestLine {
    pub fn new(method: HttpMethod, target: Box<str>, version: Box<str>) -> RequestLine {
        RequestLine {
            method,
            target,
            version
        }
    }
}

#[derive(Debug)]
pub struct Request {
    pub request_line: RequestLine,
    pub headers: Headers,
    pub body: Option<Box<str>>,
}

impl Request {
    pub fn new(request_line: RequestLine, headers: Headers, body: Option<Box<str>>) -> Request {
        Request {
            request_line,
            headers,
            body,
        }
    }

    pub fn from_string(request_string: &mut String) -> Option<Self> {
        let mut headers = Headers::new();

        let mut lines = request_string.lines();
        let mut request_line = lines.next()?.split_whitespace();
        let request_line = RequestLine {
            method: match request_line.next()? {
                method => crate::HttpMethod::from(method)?,
            },
            target: Box::from(request_line.next()?),
            version: Box::from(request_line.next()?),
        };


        loop {
            let line = lines.next().unwrap();
            if line.is_empty() {
                break
            }

            let mut header = line.splitn(2, ":");
            let key = header.next().unwrap().trim();
            let value = header.next().unwrap().trim();

            headers.insert(Box::from(key), Box::from(value));
        }

        let lines: Vec<&str> = lines.collect();
        let body: String = lines.join("\n");

        let result = Request {
            request_line,
            headers,
            body: if body.len() > 0 { Some(Box::from(body)) } else { None },
        };

        Some(result)
    }
}
