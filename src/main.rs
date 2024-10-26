use codecrafters_http_server::handler::handle_connection;
use codecrafters_http_server::router::Router;
use codecrafters_http_server::HttpMethod::{GET, POST};
use handlers::{fetch_file, handle_echo, handle_root, user_agent, write_file};
use std::env;
use std::net::TcpListener;
use std::sync::Arc;
use std::thread;

mod handlers;

#[derive(Debug)]
struct ApplicationState {
    dir: Option<String>,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_dir = match args.iter().position(|arg| arg.eq("--directory")) {
        Some(i) => args.get(i + 1),
        None => None,
    };

    let state = ApplicationState {
        dir: file_dir.cloned(),
    };

    let address = "localhost:4221";
    let router = Arc::new(
        Router::new(state)
            .add_route(GET, "/", handle_root)
            .add_route(GET, "/echo/*", handle_echo)
            .add_route(GET, "/user-agent", user_agent)
            .add_route(GET, "/files/*", fetch_file)
            .add_route(POST, "/files/*", write_file),
    );
    let listener = TcpListener::bind(address).unwrap();
    println!("listening on: {address}");

    listener.incoming().for_each(|stream| {
        let router = Arc::clone(&router);
        thread::spawn(move || {
            let stream = stream.unwrap();
            handle_connection(stream, router);
        });
    })
}
