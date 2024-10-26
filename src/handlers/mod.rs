use crate::ApplicationState;
use codecrafters_http_server::request::Request;
use codecrafters_http_server::response::{Response, StatusLine};
use std::collections::BTreeMap;
use std::fs;

pub fn handle_root(_: Vec<String>, _: &Request, _: &ApplicationState) -> Response {
    Response {
        status_line: StatusLine::new(200, Box::from("OK")),
        headers: BTreeMap::new(),
        body: None,
    }
}

pub fn handle_echo(mut args: Vec<String>, _: &Request, _: &ApplicationState) -> Response {
    let body: Box<str> = Box::from(args.remove(0));

    Response {
        status_line: StatusLine::new(200, Box::from("OK")),
        headers: BTreeMap::from([
            (Box::from("Content-Type"), Box::from("text/plain")),
            (
                Box::from("Content-Length"),
                Box::from(format!("{}", body.len())),
            ),
        ]),
        body: Some(body),
    }
}

pub fn user_agent(_: Vec<String>, request: &Request, _: &ApplicationState) -> Response {
    let body: Box<str> = match request.headers.get("user-agent") {
        Some(user_agent) => user_agent.clone(),
        None => {
            return Response {
                status_line: StatusLine::new(400, Box::from("Bad Request")),
                headers: BTreeMap::new(),
                body: None,
            }
        }
    };

    let headers = BTreeMap::from([
        (Box::from("Content-Type"), Box::from("text/plain")),
        (
            Box::from("Content-Length"),
            Box::from(format!("{}", body.len())),
        ),
    ]);

    let status_line = StatusLine::new(200, Box::from("OK"));
    Response::new(status_line, headers, Some(body))
}

pub fn fetch_file(args: Vec<String>, _: &Request, state: &ApplicationState) -> Response {
    let mut file_path = match &state.dir {
        Some(path) => path.clone(),
        None => {
            return Response::new(
                StatusLine::new(404, "Not Found".into()),
                BTreeMap::new(),
                None,
            )
        }
    };
    file_path.push_str(&format!("/{}", args.get(0).unwrap()));

    let body = match fs::read_to_string(&file_path) {
        Ok(body) => body,
        Err(_) => {
            return Response::new(
                StatusLine::new(404, "Not Found".into()),
                BTreeMap::new(),
                None,
            )
        }
    };

    let content_length = format!("{}", body.len());

    let headers = BTreeMap::from([
        ("Content-Type".into(), "application/octet-stream".into()),
        ("Content-Length".into(), content_length.into()),
    ]);

    Response::new(
        StatusLine::new(200, "OK".into()),
        headers,
        Some(body.into()),
    )
}

pub fn write_file(args: Vec<String>, request: &Request, state: &ApplicationState) -> Response {
    println!("state {:?}", &state);
    let mut file_path = match &state.dir {
        Some(path) => path.clone(),
        None => {
            return Response::new(
                StatusLine::new(404, "Not Found".into()),
                BTreeMap::new(),
                None,
            )
        }
    };
    file_path.push_str(&format!("/{}", args.get(0).unwrap()));

    let contents: String = match &request.body {
        Some(contents) => String::from_utf8_lossy(&contents).to_string(),
        None => "".into(),
    };

    fs::write(&file_path, contents).expect("Failed to write to file");

    let status_line = StatusLine::new(201, Box::from("Created"));
    Response::new(status_line, BTreeMap::new(), None)
}
