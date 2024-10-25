use crate::request::Request;
use crate::router::Router;
use std::io::{prelude::*, ErrorKind::WouldBlock};
use std::net::TcpStream;
use std::sync::Arc;

pub fn handle_connection<T>(mut stream: TcpStream, router: Arc<Router<T>>) {
    let mut buf = [0u8; 4096];
    let mut request_string = String::new();

    // TODO: Actually parse this correctly
    loop {
        match stream.read(&mut buf) {
            Ok(0) => {
                println!("Connection closed.");
                return;
            }
            Ok(n) => {
                request_string.push_str(&String::from_utf8_lossy(&buf[..n]));
            }
            Err(e) if e.kind() == WouldBlock && !request_string.is_empty() => {
                break;
            }
            Err(e) => {
                eprintln!("{e}");
            }
        };
    }

    let request = Request::from_string(&mut request_string);
    let response = router.handle(request.expect("TODO"));

    let response_string = response.to_string();

    stream.write(response_string.as_bytes()).unwrap();
}
