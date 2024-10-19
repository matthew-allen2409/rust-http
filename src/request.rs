use std::collections::HashMap;

#[derive(Debug)]
pub struct RequestLine {
    pub method: crate::HttpMethod,
    pub target: String,
    pub version: String,
}

#[derive(Debug)]
pub struct Request {
    pub request_line: RequestLine,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

impl Request {
    pub fn from_string(request_string: &mut String) -> Request {
        let mut headers = HashMap::<String, String>::new();

        let mut lines = request_string.lines();
        let mut request_line = lines.next().unwrap().split_whitespace();
        let request_line = RequestLine {
            method: match request_line.next().unwrap() {
                _ => crate::HttpMethod::GET,
            },
            target: request_line.next().unwrap().to_string(),
            version: request_line.next().unwrap().to_string(),
        };


        loop {
            let line = lines.next().unwrap();
            if line.is_empty() {
                break
            }

            let mut header = line.splitn(2, ":");
            let key = header.next().unwrap().trim().to_string();
            let value = header.next().unwrap().trim().to_string();

            headers.insert(key, value);
        }

        Request {
            request_line,
            headers,
            body: None,
        }
    }
}
