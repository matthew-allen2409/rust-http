use crate::request::{Request, RequestLine};
use crate::router::Router;
use std::collections::BTreeMap;
use std::io::{prelude::*, BufReader};
use std::net::TcpStream;
use std::sync::Arc;

static ACCEPTED_ENCODINGS: [&str; 1] = ["gzip"];

pub fn handle_connection<T>(mut stream: TcpStream, router: Arc<Router<T>>) {
    let mut buf_reader = BufReader::new(&mut stream);

    let request = parse_request(&mut buf_reader);
    let accepted_encodings = request.headers.get("accept-encoding");

    let mut response = router.handle(&request);

    match accepted_encodings {
        Some(encodings) => {
            if ACCEPTED_ENCODINGS.contains(&&**encodings) {
                response.headers.insert("Content-Encoding".into(), encodings.clone());
            }
        },
        None => ()
    }


    let response_string = response.to_string();

    stream.write(response_string.as_bytes()).unwrap();
}

fn parse_request(buf_reader: &mut BufReader<&mut TcpStream>) -> Request {
    let mut request_line = String::new();
    let mut headers: BTreeMap<Box<str>, Box<str>> = BTreeMap::new();
    let mut body = None;

    buf_reader.read_line(&mut request_line).unwrap();
    let request_line = RequestLine::from_string(request_line.trim()).expect("invalid request");

    let mut header = String::new();
    loop {
        buf_reader.read_line(&mut header).unwrap();
        let header_str = header.trim();

        if header_str.is_empty() {
            break;
        }

        let mut parts = header_str.split(':').map(|part| part.trim());
        let name = Box::from(parts.next().unwrap().to_lowercase());
        let value = Box::from(parts.next().unwrap());

        headers.insert(name, value);
        header.clear();
    }

    let content_len = match headers.get("content-length") {
        Some(value) => value.parse::<usize>().unwrap(),
        None => 0,
    };

    if content_len > 0 {
        let mut buf = vec![0; content_len];
        buf_reader.read_exact(&mut buf).unwrap();

        body = Some(buf);
    }

    Request::new(request_line, headers, body)
}

