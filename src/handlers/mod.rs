use codecrafters_http_server::response::{Response, StatusLine};
use codecrafters_http_server::{Header, Headers};

pub fn handle_root(_: Vec<String>, _: Headers) -> Response {
    Response {
        status_line: StatusLine::new(200, Box::from("OK")),
        headers: vec![],
        body: None,
    }
}

pub fn handle_echo(mut args: Vec<String>, _: Headers) -> Response {
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

pub fn user_agent(_: Vec<String>, mut headers: Headers) -> Response {
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
