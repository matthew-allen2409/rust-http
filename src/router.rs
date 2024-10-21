use crate::response::Response;
use crate::trie_tree::TrieNode;
use std::collections::VecDeque;
use crate::Handler;

#[derive(Debug)]
pub struct Router {
    pub routes: TrieNode,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: TrieNode::new(),
        }
    }

    pub fn add_route(mut self, route: &str, handler: Handler) -> Self {
        let path_vec: VecDeque<String> = route.split("/").map(str::to_string).collect();
        self.routes.add(path_vec, handler);
        self
    }

    pub fn handle_route(&self, route: String) -> Response {
        let path = route.split("/").map(String::from).collect();
        self.routes.handle(path)
    }
}

