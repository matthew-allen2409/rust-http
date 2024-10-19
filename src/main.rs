use codecrafters_http_server::handler::handle_connection;
#[allow(unused_imports)]
use std::net::{TcpListener, TcpStream};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    listener.incoming().for_each(|stream| {
        let stream = stream.unwrap();
        stream.set_nonblocking(true).unwrap();
        handle_connection(stream);
    })
}
