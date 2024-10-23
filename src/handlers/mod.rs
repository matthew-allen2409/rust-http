use codecrafters_http_server::response::{Response, StatusLine};
use codecrafters_http_server::{Header, Headers};
use crate::ApplicationState;
use std::fs;

pub fn handle_root(_: Vec<String>, _: Headers, _: &ApplicationState) -> Response {
    Response {
        status_line: StatusLine::new(200, Box::from("OK")),
        headers: vec![],
        body: None,
    }
}

pub fn handle_echo(mut args: Vec<String>, _: Headers, _: &ApplicationState) -> Response {
    let body: Box<str> = Box::from(args.remove(0));

    Response {
        status_line: StatusLine::new(200, Box::from("OK")),
        headers: vec![
            Header::new(Box::from("Content-Type"), Box::from("text/plain")),
            Header::new(Box::from("Content-Length"), Box::from(format!("{}", body.len()))),
        ],
        body: Some(body),
    }
}

pub fn user_agent(_: Vec<String>, mut headers: Headers, _: &ApplicationState) -> Response {
    let body = match headers.remove("User-Agent") {
        Some(user_agent) => user_agent,
        None => return Response {
            status_line: StatusLine::new(400, Box::from("Bad Request")),
            headers: vec![],
            body: None,
            }
    };

    let headers: Vec<Header> = Vec::from([
        Header::new(Box::from("Content-Type"), Box::from("text/plain")),
        Header::new(Box::from("Content-Length"), Box::from(format!("{}", body.len()))),
    ]);

    let status_line = StatusLine::new(200, Box::from("OK"));
    Response::new(
        status_line,
        headers,
        Some(body),
    )
}


pub fn fetch_file(args: Vec<String>, _: Headers, state: &ApplicationState) -> Response {
    let mut file_path = state.dir.clone();
    file_path.push_str(&format!("/{}", args.get(0).unwrap()));

    let body = match fs::read_to_string(&file_path) {
        Ok(body) => body,
        Err(_) => {
            return Response::new(
                StatusLine::new(404, format!("File not found: {}", &file_path).into()),
                vec![],
                None,
            )
        }
    };

    let content_length = format!("{}", body.len());

    let headers = vec![
        Header::new("Content-Type".into(), "application/octet-stream".into()),
        Header::new("Content-Length".into(), content_length.into())
    ];

    Response::new(
        StatusLine::new(200, "OK".into()),
        headers,
        Some(body.into()),
    )
}
