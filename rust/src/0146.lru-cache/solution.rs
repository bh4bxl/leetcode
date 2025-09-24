// Created by bh4bxl at 2025/09/22 13:07
// leetgo: 1.4.15
// https://leetcode.com/problems/lru-cache/

use anyhow::Result;
use leetgo_rs::*;

// @lc code=begin
use std::{cell::RefCell, collections::HashMap, rc::Rc};

struct Node {
    key: i32,
    val: i32,
    prev: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(key: i32, val: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            key,
            val,
            prev: None,
            next: None,
        }))
    }
}

struct LRUCache {
    cap: usize,
    map: HashMap<i32, Rc<RefCell<Node>>>,
    head: Rc<RefCell<Node>>,
    tail: Rc<RefCell<Node>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {
    fn new(capacity: i32) -> Self {
        let head = Node::new(0, 0);
        let tail = Node::new(0, 0);
        head.borrow_mut().next = Some(tail.clone());
        tail.borrow_mut().prev = Some(head.clone());
        LRUCache {
            cap: capacity as usize,
            map: HashMap::new(),
            head,
            tail,
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if let Some(n) = self.map.get(&key) {
            let val = n.borrow().val;
            self.remove(n.clone());
            self.insert_front(n.clone());
            val
        } else {
            -1
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if let Some(n) = self.map.get(&key) {
            n.borrow_mut().val = value;
            self.remove(n.clone());
            self.insert_front(n.clone());
        } else {
            if self.map.len() == self.cap {
                let lru = self.tail.borrow().prev.clone().unwrap();
                self.remove(lru.clone());
                self.map.remove(&lru.borrow().key);
            }
            let new_node = Node::new(key, value);
            self.insert_front(new_node.clone());
            self.map.insert(key, new_node);
        }
    }

    fn remove(&self, node: Rc<RefCell<Node>>) {
        let prev = node.borrow().prev.clone().unwrap();
        let next = node.borrow().next.clone().unwrap();
        prev.borrow_mut().next = Some(next.clone());
        next.borrow_mut().prev = Some(prev);
    }

    fn insert_front(&self, node: Rc<RefCell<Node>>) {
        let first = self.head.borrow().next.clone().unwrap();
        node.borrow_mut().prev = Some(self.head.clone());
        node.borrow_mut().next = Some(first.clone());
        self.head.borrow_mut().next = Some(node.clone());
        first.borrow_mut().prev = Some(node);
    }
}

// @lc code=end

fn main() -> Result<()> {
    let ops: Vec<String> = deserialize(&read_line()?)?;
    let params = split_array(&read_line()?)?;
    let mut output = Vec::with_capacity(ops.len());
    output.push("null".to_string());

    let constructor_params = split_array(&params[0])?;
    let capacity: i32 = deserialize(&constructor_params[0])?;
    #[allow(unused_mut)]
    let mut obj = LRUCache::new(capacity);

    for i in 1..ops.len() {
        match ops[i].as_str() {
            "get" => {
                let method_params = split_array(&params[i])?;
                let key: i32 = deserialize(&method_params[0])?;
                let ans: i32 = obj.get(key).into();
                output.push(serialize(ans)?);
            }
            "put" => {
                let method_params = split_array(&params[i])?;
                let key: i32 = deserialize(&method_params[0])?;
                let value: i32 = deserialize(&method_params[1])?;
                obj.put(key, value);
                output.push("null".to_string());
            }
            _ => panic!("unknown op"),
        }
    }

    println!("\noutput: {}", join_array(output));
    Ok(())
}
