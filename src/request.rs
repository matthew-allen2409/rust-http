use std::collections::HashMap;

#[derive(Debug)]
pub struct RequestLine {
    pub method: crate::HttpMethod,
    pub target: Box<str>,
    pub version: Box<str>,
}

#[derive(Debug)]
pub struct Request {
    pub request_line: RequestLine,
    pub headers: HashMap<Box<str>, Box<str>>,
    pub body: Option<String>,
}

impl Request {
    pub fn from_string(request_string: &mut String) -> Request {
        let mut headers = HashMap::<Box<str>, Box<str>>::new();

        let mut lines = request_string.lines();
        let mut request_line = lines.next().unwrap().split_whitespace();
        let request_line = RequestLine {
            method: match request_line.next().unwrap() {
                _ => crate::HttpMethod::GET,
            },
            target: Box::from(request_line.next().unwrap()),
            version: Box::from(request_line.next().unwrap()),
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

        Request {
            request_line,
            headers,
            body: None,
        }
    }
}
