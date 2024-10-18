use std::io::prelude::*;

#[allow(unused_imports)]
use std::net::{ TcpListener, TcpStream };

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    listener.incoming().for_each(|stream| {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                write_header(&mut stream).unwrap()

            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    })
}

fn write_header(stream: &mut TcpStream) -> std::io::Result<()> {
    match stream.write(b"HTTP/1.1 200 OK\r\n\r\n") {
        Ok(bytes_written) => println!("Bytes written: {}", bytes_written),
        Err(e) => return Err(e)
    };

    Ok(())
}
