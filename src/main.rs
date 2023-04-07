#![allow(unused)]

// https://youtu.be/wDnI8liqD-o?t=890

use std::{collections::VecDeque, fmt::Display};

#[derive(Default)]
struct Node {
    children: Vec<Node>,
    key: Option<char>,
    value: Option<String>,
    count: usize, // How many times a string has been inserted
}

impl Node {
    fn new() -> Self {
        Node {
            ..Default::default()
        }
    }

    fn with_key(c: char) -> Self {
        Node {
            key: Some(c),
            ..Default::default()
        }
    }
}

struct Trie {
    root: Node, // the root Node doesn't have a key
}

impl Trie {
    fn new() -> Self {
        Trie { root: Node::new() }
    }

    fn insert(&mut self, s: &str) {
        let mut current = &mut self.root;

        for c in s.chars() {
            match current.children.binary_search_by(|f| f.key.cmp(&Some(c))) {
                Ok(i) => {
                    current = &mut current.children[i];
                }
                Err(i) => {
                    current.children.insert(i, Node::with_key(c));
                    current = &mut current.children[i];
                }
            }
        }

        current.count += 1;
        current.value.replace(s.to_string());
    }

    fn exists(&self, s: &str) -> bool {
        let mut current = &self.root;

        for c in s.chars() {
            match current.children.binary_search_by(|f| f.key.cmp(&Some(c))) {
                Ok(i) => {
                    current = &current.children[i];
                }
                Err(_) => return false,
            }
        }

        current.count > 0 // this string has been inserted
    }

    fn search(&self, s: &str) -> Vec<String> {
        let mut current = &self.root;

        for c in s.chars() {
            match current.children.binary_search_by(|f| f.key.cmp(&Some(c))) {
                Ok(i) => {
                    current = &current.children[i];
                }
                Err(_) => return Vec::new(),
            }
        }

        let mut results = Vec::new();
        let mut queue = Vec::new();
        queue.push(current);
        while let Some(c) = queue.pop() {
            for child in c.children.iter() {
                queue.push(&child);
            }

            let count = c.count;

            if count > 0 {
                let value = c.value.as_ref().unwrap();
                results.push((count, value));
            }
        }

        results.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(b.1)));
        results.iter().map(|m| m.1.clone()).collect()
    }
}

impl Display for Trie {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut queue = VecDeque::new();
        let root = &self.root;
        queue.push_back(root);

        while !queue.is_empty() {
            for _ in 0..queue.len() {
                if let Some(node) = queue.pop_front() {
                    for c in node.children.iter() {
                        let r = write!(f, "{} ", &c.key.unwrap());
                        if r.is_err() {
                            return r;
                        }
                        if c.children.len() > 0 {
                            queue.push_back(&c);
                        }
                    }
                }
            }

            if queue.len() > 0 {
                let r = writeln!(f);
                if r.is_err() {
                    return r;
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn run() {
        let mut trie = Trie::new();

        trie.insert("a");
        trie.insert("to");
        trie.insert("tea");
        trie.insert("apples");
        trie.insert("an");
        trie.insert("test");
        trie.insert("tea");

        assert!(trie.exists("test"));
        assert!(trie.exists("to"));
        assert!(trie.exists("tea"));
        assert!(!trie.exists("airplane"));

        println!("{}", trie);

        assert_eq!(trie.search("te"), vec!["tea", "test"]);
        assert_eq!(trie.search("a"), vec!["a", "an", "apples"]);

        trie.insert("test");
        trie.insert("test");

        assert_eq!(trie.search("te"), vec!["test", "tea"]);
    }
}

fn main() {}
