use crate::response::{Response, StatusLine};
use crate::Headers;
use crate::Handler;
use std::collections::{BTreeMap, VecDeque};
use std::sync::Arc;

#[derive(Debug)]
pub struct Router {
    pub routes: PathNode,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: PathNode::new(),
        }
    }

    pub fn add_route(mut self, route: &str, handler: Handler) -> Self {
        let path_vec: VecDeque<String> = route.split("/").map(str::to_string).collect();
        self.routes.add_route(path_vec, handler);
        self
    }

    pub fn handle_route(&self, route: String, headers: Headers) -> Response {
        let path = route.split("/").map(String::from).collect();
        self.routes.handle(path, headers)
    }
}

#[derive(Debug, PartialEq)]
pub struct PathNode {
    pub children: BTreeMap<String, PathNode>,
    pub handler: Option<Arc<Handler>>,
}

impl PathNode {
    pub fn new() -> Self {
        Self {
            children: BTreeMap::new(),
            handler: None,
        }
    }

    pub fn add_route(&mut self, mut path: VecDeque<String>, handler: Handler) {
        let path_element = match path.pop_front() {
            Some(element) => element,
            None => {
                self.handler = Some(Arc::new(handler));
                return;
            }
        };

        let child: &mut PathNode = match self.children.get_mut(&path_element) {
            Some(child) => child,
            None => &mut self.children.entry(path_element).or_insert(PathNode::new()),
        };
        child.add_route(path, handler);
    }

    pub fn handle(&self, path: VecDeque<String>, headers: Headers) -> Response {
        self.find(path, Vec::new(), headers)
    }

    fn find(&self, mut path: VecDeque<String>, mut arg_acc: Vec<String>, headers: Headers) -> Response {
        let path_element = match path.pop_front() {
            Some(element) => element,
            None => {
                return match &self.handler {
                    Some(handler) => handler(arg_acc, headers),
                    None => handle_not_found(),
                }
            }
        };

        return match self.children.get(&path_element) {
            Some(child) => child.find(path, arg_acc, headers),
            None if self.children.contains_key("*") => {
                arg_acc.push(path_element);
                self.children.get("*").unwrap().find(path, arg_acc, headers)
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
mod tests {
    use super::*;
    use crate::response::{Response, StatusLine};
    use std::collections::VecDeque;

    const DUMMY_HEADERS: Headers = Headers::new();
    fn dummy(_: Vec<String>, _: Headers) -> Response {
        Response {
            status_line: StatusLine {
                version: Box::from(""),
                status_code: 200,
                status_text: Box::from("OK"),
            },
            headers: vec![],
            body: None,
        }
    }


    #[test]
    fn new_expect_default_trie_node() {
        let expected = PathNode {
            children: BTreeMap::new(),
            handler: None,
        };

        let result = PathNode::new();

        assert_eq!(expected, result);
    }

    #[test]
    fn add_root_expect_root_handler() {
        let path = VecDeque::new();

        let expected = PathNode {
            children: BTreeMap::new(),
            handler: Some(Arc::new(dummy)),
        };

        let mut result = PathNode::new();
        result.add_route(path, dummy);

        assert_eq!(expected, result);
    }

    #[test]
    fn add_expect_correct_tree() {
        let path = VecDeque::from(["echo".to_string(), "hello".to_string()]);

        let hello = PathNode {
            children: BTreeMap::new(),
            handler: Some(Arc::new(dummy)),
        };
        let echo = PathNode {
            children: BTreeMap::from([("hello".to_string(), hello)]),
            handler: None,
        };
        let expected = PathNode {
            children: BTreeMap::from([("echo".to_string(), echo)]),
            handler: None,
        };

        let mut result = PathNode::new();
        result.add_route(path, dummy);

        assert_eq!(expected, result)
    }

    #[test]
    fn hanlde_root_inserted_expect_handler() {
        let tree = PathNode {
            children: BTreeMap::new(),
            handler: Some(Arc::new(dummy)),
        };
        let path = VecDeque::new();
        let result = tree.handle(path, DUMMY_HEADERS);

        assert_eq!(dummy(vec![], Headers::new()), result);
    }

    #[test]
    fn handle_path_inserted_expect_handler() {
        let path = VecDeque::from(["echo".to_string(), "hello".to_string()]);

        let hello = PathNode {
            children: BTreeMap::new(),
            handler: Some(Arc::new(dummy)),
        };
        let echo = PathNode {
            children: BTreeMap::from([("hello".to_string(), hello)]),
            handler: None,
        };
        let tree = PathNode {
            children: BTreeMap::from([("echo".to_string(), echo)]),
            handler: None,
        };

        let result = tree.handle(path, DUMMY_HEADERS);

        assert_eq!(dummy(vec![], Headers::new()), result);
    }
}
