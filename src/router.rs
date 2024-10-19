use crate::response::{ StatusLine, Response };

pub fn handle_route(route: String) -> Response {
    match route.as_str() {
        "/" => handle_root(),
        _ => handle_not_found(),
    }
}

fn handle_root() -> Response {
    Response {
        status_line: StatusLine {
            version: "HTTP/1.1".to_string(),
            status_code: 200,
            status_text: "OK".to_string(),
        },
        headers: vec!(),
        body: None,
    }
}

fn handle_not_found() -> Response {
    Response {
        status_line: StatusLine {
            version: "HTTP/1.1".to_string(),
            status_code: 404,
            status_text: "Not Found".to_string(),
        },
        headers: vec!(),
        body: None
    }
}
