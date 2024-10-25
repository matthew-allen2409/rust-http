use crate::request::{Request, RequestLine};
use crate::response::{Response, StatusLine};
use crate::router::{Handler, PathNode};
use crate::Headers;
use crate::HttpMethod;
use std::collections::{BTreeMap, VecDeque};
use std::sync::Arc;

const DUMMY_HEADERS: Headers = Headers::new();
fn dummy(_: Vec<String>, _: Request, _: &String) -> Response {
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

fn dummy_request(str: &str) -> Request {
    let request_line = RequestLine::new(
        HttpMethod::GET,
        Box::from(str),
        Box::from("HTTP/1.1"),
    );

    Request::new(request_line, DUMMY_HEADERS, None)
}

#[test]
fn new_expect_default_path_node() {
    let expected = PathNode::<String> {
        children: BTreeMap::new(),
        handlers: BTreeMap::new(),
    };

    let result = PathNode::new();

    assert_eq!(expected, result);
}

#[test]
fn add_root_expect_root_handler() {
    let path = VecDeque::new();

    let mut expected_handlers: BTreeMap<HttpMethod, Arc<Handler<String>>> = BTreeMap::new();
    expected_handlers.insert(HttpMethod::GET, Arc::new(dummy));

    let expected = PathNode {
        children: BTreeMap::new(),
        handlers: expected_handlers,
    };

    let mut result = PathNode::new();
    result.add_route(HttpMethod::GET, path, dummy);

    assert_eq!(expected, result);
}

#[test]
fn add_expect_correct_tree() {
    let path = VecDeque::from(["echo".to_string(), "hello".to_string()]);

    let mut expected_handlers: BTreeMap<HttpMethod, Arc<Handler<String>>> = BTreeMap::new();
    expected_handlers.insert(HttpMethod::GET, Arc::new(dummy));

    let hello = PathNode::<String> {
        children: BTreeMap::new(),
        handlers: expected_handlers,
    };
    let echo = PathNode::<String> {
        children: BTreeMap::from([("hello".to_string(), hello)]),
        handlers: BTreeMap::new(),
    };
    let expected = PathNode::<String> {
        children: BTreeMap::from([("echo".to_string(), echo)]),
        handlers: BTreeMap::new(),
    };

    let mut result = PathNode::new();
    result.add_route(HttpMethod::GET, path, dummy);

    assert_eq!(expected, result)
}

#[test]
fn handle_root_inserted_expect_handler() {
    let mut expected_handlers: BTreeMap<HttpMethod, Arc<Handler<String>>> = BTreeMap::new();
    expected_handlers.insert(HttpMethod::GET, Arc::new(dummy));

    let tree = PathNode::<String> {
        children: BTreeMap::new(),
        handlers: expected_handlers,
    };
    let result = tree.handle(dummy_request("/"), &String::from("foo"));

    assert_eq!(dummy(vec![], dummy_request("/"), &String::from("foo")), result);
}

#[test]
fn handle_path_inserted_expect_handler() {
    let mut expected_handlers: BTreeMap<HttpMethod, Arc<Handler<String>>> = BTreeMap::new();
    expected_handlers.insert(HttpMethod::GET, Arc::new(dummy));

    let hello = PathNode::<String> {
        children: BTreeMap::new(),
        handlers: expected_handlers,
    };
    let echo = PathNode::<String> {
        children: BTreeMap::from([("hello".to_string(), hello)]),
        handlers: BTreeMap::new(),
    };
    let tree = PathNode::<String> {
        children: BTreeMap::from([("echo".to_string(), echo)]),
        handlers: BTreeMap::new(),
    };

    let result = tree.handle(dummy_request("/echo/hello"), &String::from("actual"));
    println!("tree: {:?}", tree);
    assert_eq!(
        dummy(vec![], dummy_request("/echo/hello/"), &String::from("expected")),
        result
    );
}
