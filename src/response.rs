use crate::Header;

pub struct StatusLine {
    pub version: String,
    pub status_code: u16,
    pub status_text: String,
}

pub struct Response {
    pub status_line: StatusLine,
    pub headers: Vec<Header>,
    pub body: Option<String>,
}

impl Response {
    pub fn to_string(&self) -> String {
        format!(
            "{} {} {}\r\n\r\n",
            self.status_line.version, self.status_line.status_code, self.status_line.status_text,
        )
    }
}
