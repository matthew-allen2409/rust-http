use crate::request::{Request, RequestLine};
use crate::response::Response;
use crate::router::Router;
use std::collections::BTreeMap;
use std::io::{prelude::*, BufReader};
use std::net::TcpStream;
use std::sync::Arc;
use flate2::write::GzEncoder;
use flate2::Compression;

static SUPPORTED_ENCODINGS: [&str; 1] = ["gzip"];

pub fn handle_connection<T>(mut stream: TcpStream, router: Arc<Router<T>>) { let mut buf_reader = BufReader::new(&mut stream);

    let request = parse_request(&mut buf_reader);
    let mut response = router.handle(&request);
    process_accepted_encodings(&request, &mut response);

    let response_bytes = response.to_bytes();
    stream.write(&response_bytes).unwrap();
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

fn process_accepted_encodings(request: &Request, response: &mut Response) {
    let accepted_encodings = request.headers.get("accept-encoding");

    match accepted_encodings {
        Some(encodings) => {
            for encoding in encodings.split(',').map(str::trim) {
                if SUPPORTED_ENCODINGS.contains(&encoding) {
                    response
                        .headers
                        .insert("Content-Encoding".into(), Box::from(encoding));

                    let body = match &response.body {
                        Some(body) => body,
                        None => break,
                    };

                    let encoded_body = encode_data(body);
                    let content_len = encoded_body.len().to_string();
                    response.body = Some(encoded_body);

                    response.headers.insert(Box::from("Content-Length"), Box::from(content_len));

                    break;
                }
            }
        }
        None => (),
    }
}

fn encode_data(body: &Vec<u8>) -> Vec<u8> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());

    encoder.write_all(&body).unwrap();

    let encoded_string = encoder.finish().unwrap();

    Vec::from(encoded_string)
}
