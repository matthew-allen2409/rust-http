use crate::request::Request;
use crate::response::{Response, StatusLine};
use crate::HttpMethod;
use std::collections::{BTreeMap, VecDeque};
use std::sync::Arc;

pub type Handler<T> = fn(Vec<String>, Request, &T) -> Response;

#[derive(Debug, PartialEq)]
pub struct PathNode<T> {
    pub children: BTreeMap<String, PathNode<T>>,
    pub handlers: BTreeMap<HttpMethod, Arc<Handler<T>>>,
}

impl<T> PathNode<T> {
    pub fn new() -> Self {
        Self {
            children: BTreeMap::new(),
            handlers: BTreeMap::new(),
        }
    }

    pub fn add_route(
        &mut self,
        method: HttpMethod,
        mut path: VecDeque<String>,
        handler: Handler<T>,
    ) {
        let path_element = match path.pop_front() {
            Some(element) => element,
            None => {
                self.handlers.insert(method, Arc::new(handler));
                return;
            }
        };

        let child: &mut PathNode<T> = match self.children.get_mut(&path_element) {
            Some(child) => child,
            None => &mut self.children.entry(path_element).or_insert(PathNode::new()),
        };
        child.add_route(method, path, handler);
    }

    pub fn handle(&self, request: Request, state: &T) -> Response {
        let path: VecDeque<String> = request
            .request_line
            .target
            .split('/')
            .filter(|it| *it != "")
            .map(str::to_string)
            .collect();
        self.find(Vec::new(), path, request, state)
    }

    fn find(
        &self,
        mut arg_acc: Vec<String>,
        mut path: VecDeque<String>,
        request: Request,
        state: &T,
    ) -> Response {
        let path_element = match path.pop_front() {
            Some(element) => element,
            None => {
                println!("Handlers: {:?}", &self.handlers);
                println!("Request: {:?}", &request);
                return match &self.handlers.get(&request.request_line.method) {
                    Some(handler) => handler(arg_acc, request, state),
                    None => handle_not_found(),
                }
            }
        };

        return match self.children.get(&path_element) {
            Some(child) => child.find(arg_acc, path, request, state),
            None if self.children.contains_key("*") => {
                println!("wildcard: {}", path_element);
                arg_acc.push(path_element);
                self.children
                    .get("*")
                    .unwrap()
                    .find(arg_acc, path, request, state)
            }
            None => handle_not_found(),
        };
    }
}

fn handle_not_found() -> Response {
    Response {
        status_line: StatusLine {
            version: Box::from("HTTP/1.1"),
            status_code: 404,
            status_text: Box::from("Not Found"),
        },
        headers: vec![],
        body: None,
    }
}

#[cfg(test)]
mod tests;
