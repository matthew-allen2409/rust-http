use codecrafters_http_server::handler::handle_connection;
use codecrafters_http_server::router::Router;
use handlers::{handle_echo, handle_root, user_agent};
use std::net::TcpListener;
use std::sync::Arc;
use std::thread;

mod handlers;

fn main() {
    let address = "localhost:4221";
    let router = Arc::new(
        Router::new()
            .add_route("/", handle_root)
            .add_route("/echo/*", handle_echo)
            .add_route("/user-agent", user_agent),
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
