use crate::Handler;
use crate::response::{ Response, StatusLine };
use std::collections::{BTreeMap, VecDeque};
use std::sync::Arc;

#[derive(Debug, PartialEq)]
pub struct TrieNode {
    pub children: BTreeMap<String, TrieNode>,
    pub handler: Option<Arc<Handler>>,
}

impl TrieNode {
    pub fn new() -> Self {
        Self {
            children: BTreeMap::new(),
            handler: None,
        }
    }

    pub fn add(&mut self, mut path: VecDeque<String>, handler: Handler) {
        let path_element = match path.pop_front() {
            Some(element) => element,
            None => {
                self.handler = Some(Arc::new(handler));
                return;
            }
        };

        let child: &mut TrieNode = match self.children.get_mut(&path_element) {
            Some(child) => child,
            None => &mut self.children.entry(path_element).or_insert(TrieNode::new()),
        };
        child.add(path, handler);
    }

    pub fn handle(
        &self,
        path: VecDeque<String>,
    ) -> Response {
        self.find(path, Vec::new())
    }

    fn find(
        &self,
        mut path: VecDeque<String>,
        mut arg_acc: Vec<String>,
    ) -> Response {
        let path_element = match path.pop_front() {
            Some(element) => element,
            None => {
                return match &self.handler {
                    Some(handler) => {
                        handler(arg_acc)
                    }
                    None => handle_not_found(),
                }
            }
        };

        return match self.children.get(&path_element) {
            Some(child) => child.find(path, arg_acc),
            None if self.children.contains_key("*") => {
                arg_acc.push(path_element);
                self.children.get("*").unwrap().find(path, arg_acc)
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

    fn dummy(_: Vec<String>) -> Response {
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
        let expected = TrieNode {
            children: BTreeMap::new(),
            handler: None,
        };

        let result = TrieNode::new();

        assert_eq!(expected, result);
    }

    #[test]
    fn add_root_expect_root_handler() {
        let path = VecDeque::new();

        let expected = TrieNode {
            children: BTreeMap::new(),
            handler: Some(Arc::new(dummy)),
        };

        let mut result = TrieNode::new();
        result.add(path, dummy);

        assert_eq!(expected, result);
    }

    #[test]
    fn add_expect_correct_tree() {
        let path = VecDeque::from(["echo".to_string(), "hello".to_string()]);

        let hello = TrieNode {
            children: BTreeMap::new(),
            handler: Some(Arc::new(dummy)),
        };
        let echo = TrieNode {
            children: BTreeMap::from([("hello".to_string(), hello)]),
            handler: None,
        };
        let expected = TrieNode {
            children: BTreeMap::from([("echo".to_string(), echo)]),
            handler: None,
        };

        let mut result = TrieNode::new();
        result.add(path, dummy);

        assert_eq!(expected, result)
    }

    #[test]
    #[ignore]
    fn hanlde_root_inserted_expect_handler() {
        let tree = TrieNode {
            children: BTreeMap::new(),
            handler: Some(Arc::new(dummy)),
        };
        let path = VecDeque::new();
        let _result = tree.handle(path);

        // result.unwrap()();
    }

    #[test]
    #[ignore]
    fn handle_path_inserted_expect_handler() {
        let path = VecDeque::from(["echo".to_string(), "hello".to_string()]);

        let hello = TrieNode {
            children: BTreeMap::new(),
            handler: Some(Arc::new(dummy)),
        };
        let echo = TrieNode {
            children: BTreeMap::from([("hello".to_string(), hello)]),
            handler: None,
        };
        let tree = TrieNode {
            children: BTreeMap::from([("echo".to_string(), echo)]),
            handler: None,
        };

        let _result = tree.handle(path);

        // result.unwrap()();
    }
}
