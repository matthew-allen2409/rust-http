use crate::request::Request;
use std::io::{prelude::*, ErrorKind::WouldBlock};
use std::net::TcpStream;
use crate::router::handle_route;

pub fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0u8; 4096];
    let mut request_string = String::new();

    loop {
        match stream.read(&mut buf) {
            Ok(0) => {
                println!("Connection closed.");
                return;
            }
            Ok(n) => {
                request_string.push_str(&String::from_utf8_lossy(&buf[..n]));
            }
            Err(e) if e.kind() == WouldBlock && !request_string.is_empty()=> {
                break;
            }
            Err(e) => {
                eprintln!("{e}");
            }
        };
    }

    println!("Request_string: {}", &request_string);

    let request = Request::from_string(&mut request_string);

    let response = handle_route(request.request_line.target);

    stream.write(response.to_string().as_bytes()).unwrap();
}
