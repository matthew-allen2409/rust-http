use crate::Header;

#[derive(Debug)]
pub struct StatusLine {
    pub version: Box<str>,
    pub status_code: u16,
    pub status_text: Box<str>,
}

impl StatusLine {
    pub fn to_string(&self) -> String {
        format!(
            "{} {} {}\r\n",
            self.version, self.status_code, self.status_text,
        )
    }
}

#[derive(Debug)]
pub struct Response {
    pub status_line: StatusLine,
    pub headers: Vec<Header>,
    pub body: Option<Box<str>>,
}

impl Response {
    pub fn to_string(&self) -> String {
        let status_line_string = self.status_line.to_string();

        let mut headers_string = String::new();
        self.headers.iter().for_each(|header| {
            headers_string.push_str(&header.to_string());
        });

        match &self.body {
            Some(body) => format!("{}{}\r\n{}", status_line_string, headers_string, body),
            None => format!("{}{}\r\n", status_line_string, headers_string)
        }
    }
}
