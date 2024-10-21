use codecrafters_http_server::handler::handle_connection;
use codecrafters_http_server::response::{Response, StatusLine};
use codecrafters_http_server::Header;
use codecrafters_http_server::router::Router;
use std::net::TcpListener;
use std::sync::Arc;
use std::thread;

fn main() {
    let address = "localhost:4221";
    let router = Arc::new(
        Router::new()
            .add_route("/", handle_root)
            .add_route("/echo/*", handle_echo),
    );
    let listener = TcpListener::bind(address).unwrap();
    println!("listening on: {address}");

    listener.incoming().for_each(|stream| {
        let router = Arc::clone(&router);
        thread::spawn(move || {
            let stream = stream.unwrap();
            stream.set_nonblocking(true).unwrap();
            handle_connection(stream, router);
        });
    })
}

pub fn handle_root(_: Vec<String>) -> Response {
    Response {
        status_line: StatusLine {
            version: Box::from("HTTP/1.1"),
            status_code: 200,
            status_text: Box::from("OK"),
        },
        headers: vec![],
        body: None,
    }
}

pub fn handle_echo(mut args: Vec<String>) -> Response {
    let body: Box<str> = Box::from(args.remove(0));

    Response {
        status_line: StatusLine {
            version: Box::from("HTTP/1.1"),
            status_code: 200,
            status_text: Box::from("OK"),
        },
        headers: vec![
            Header::new(Box::from("Content-Type"), Box::from("text/plain")),
            Header::new(Box::from("Content-Length"), Box::from(format!("{}", body.len()))),
        ],
        body: Some(body),
    }
}
