use crate::Headers;

#[derive(Debug, PartialEq)]
pub struct Response {
    pub status_line: StatusLine,
    pub headers: Headers,
    pub body: Option<Box<str>>,
}

impl Response {
    pub fn new(status_line: StatusLine, headers: Headers, body: Option<Box<str>>) -> Response {
        Response {
            status_line,
            headers,
            body,
        }
    }

    pub fn to_string(&self) -> String {
        let status_line_string = self.status_line.to_string();

        let mut headers_string = String::new();
        self.headers.iter().for_each(|(key, value)| {
            headers_string.push_str(&format!("{key}: {value}\r\n"));
        });

        match &self.body {
            Some(body) => format!("{}{}\r\n{}", status_line_string, headers_string, body),
            None => format!("{}{}\r\n", status_line_string, headers_string)
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct StatusLine {
    pub version: Box<str>,
    pub status_code: u16,
    pub status_text: Box<str>,
}

impl StatusLine {
    pub fn new(status_code: u16, status_text: Box<str>) -> Self {
        Self {
            version: Box::from("HTTP/1.1"),
            status_code,
            status_text,
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "{} {} {}\r\n",
            self.version, self.status_code, self.status_text,
        )
    }
}
