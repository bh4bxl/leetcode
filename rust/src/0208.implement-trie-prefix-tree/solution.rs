// Created by bh4bxl at 2025/10/21 13:41
// leetgo: 1.4.15
// https://leetcode.com/problems/implement-trie-prefix-tree/

use std::fmt::Write;

use anyhow::Result;
use leetgo_rs::*;

// @lc code=begin

#[derive(Default)]
struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    is_end: bool,
}

impl TrieNode {
    fn new() -> Self {
        Default::default()
    }
}

struct Trie {
    root: TrieNode,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::new(),
        }
    }

    fn insert(&mut self, word: String) {
        let mut node = &mut self.root;
        for ch in word.chars() {
            let idx = (ch as u8 - b'a') as usize;
            node = node.children[idx].get_or_insert_with(|| Box::new(TrieNode::new()));
        }
        node.is_end = true;
    }

    fn search(&self, word: String) -> bool {
        let mut node = &self.root;
        for ch in word.chars() {
            let idx = (ch as u8 - b'a') as usize;
            match &node.children[idx] {
                Some(child) => node = child.as_ref(),
                None => return false,
            }
        }
        node.is_end
    }

    fn starts_with(&self, prefix: String) -> bool {
        let mut node = &self.root;
        for ch in prefix.chars() {
            let idx = (ch as u8 - b'a') as usize;
            match &node.children[idx] {
                Some(child) => node = child.as_ref(),
                None => return false,
            }
        }
        true
    }
}

// @lc code=end

fn main() -> Result<()> {
    let ops: Vec<String> = deserialize(&read_line()?)?;
    let params = split_array(&read_line()?)?;
    let mut output = Vec::with_capacity(ops.len());
    output.push("null".to_string());

    #[allow(unused_mut)]
    let mut obj = Trie::new();

    for i in 1..ops.len() {
        match ops[i].as_str() {
            "insert" => {
                let method_params = split_array(&params[i])?;
                let word: String = deserialize(&method_params[0])?;
                obj.insert(word);
                output.push("null".to_string());
            }
            "search" => {
                let method_params = split_array(&params[i])?;
                let word: String = deserialize(&method_params[0])?;
                let ans: bool = obj.search(word).into();
                output.push(serialize(ans)?);
            }
            "startsWith" => {
                let method_params = split_array(&params[i])?;
                let prefix: String = deserialize(&method_params[0])?;
                let ans: bool = obj.starts_with(prefix).into();
                output.push(serialize(ans)?);
            }
            _ => panic!("unknown op"),
        }
    }

    println!("\noutput: {}", join_array(output));
    Ok(())
}
