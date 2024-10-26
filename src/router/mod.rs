use crate::request::Request;
use crate::response::Response;
use crate::HttpMethod;
use path_node::{Handler, PathNode};
use std::collections::VecDeque;

mod path_node;

#[derive(Debug)]
pub struct Router<T> {
    pub routes: PathNode<T>,
    pub state: T,
}

impl<T> Router<T> {
    pub fn new(state: T) -> Self {
        Self {
            routes: PathNode::new(),
            state,
        }
    }

    pub fn add_route(mut self, method: HttpMethod, route: &str, handler: Handler<T>) -> Self {
        let path_vec: VecDeque<String> = route
            .trim()
            .split('/')
            .filter(|it| *it != "")
            .map(str::to_string)
            .collect();
        self.routes.add_route(method, path_vec, handler);
        self
    }

    pub fn handle(&self, request: &Request) -> Response {
        self.routes.handle(request, &self.state)
    }
}
