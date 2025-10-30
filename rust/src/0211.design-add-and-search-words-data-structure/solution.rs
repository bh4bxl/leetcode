// Created by bh4bxl at 2025/10/27 13:11
// leetgo: 1.4.15
// https://leetcode.com/problems/design-add-and-search-words-data-structure/

use std::char;

use anyhow::Result;
use leetgo_rs::*;

// @lc code=begin

#[derive(Default)]
struct TrieNode {
    children: [Option<Box<TrieNode>>; 26],
    is_end: bool,
}

impl TrieNode {
    fn new() -> TrieNode {
        Default::default()
    }
}

struct WordDictionary {
    root: TrieNode,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl WordDictionary {
    fn new() -> Self {
        WordDictionary {
            root: TrieNode::new(),
        }
    }

    fn add_word(&mut self, word: String) {
        let mut node = &mut self.root;
        for ch in word.chars() {
            let idx = (ch as u8 - b'a') as usize;
            node = node.children[idx].get_or_insert_with(|| Box::new(TrieNode::new()));
        }
        node.is_end = true;
    }

    fn search(&self, word: String) -> bool {
        fn dfs(node: &TrieNode, chars: &[char], index: usize) -> bool {
            if index == chars.len() {
                return node.is_end;
            }

            let ch = chars[index];
            if ch == '.' {
                for child in node.children.iter().flatten() {
                    if dfs(child, chars, index + 1) {
                        return true;
                    }
                }
                false
            } else {
                let idx = (ch as u8 - b'a') as usize;
                match &node.children[idx] {
                    Some(child) => dfs(child, chars, index + 1),
                    None => false,
                }
            }
        }

        dfs(&self.root, &word.chars().collect::<Vec<_>>(), 0)
    }
}

// @lc code=end

fn main() -> Result<()> {
    let ops: Vec<String> = deserialize(&read_line()?)?;
    let params = split_array(&read_line()?)?;
    let mut output = Vec::with_capacity(ops.len());
    output.push("null".to_string());

    #[allow(unused_mut)]
    let mut obj = WordDictionary::new();

    for i in 1..ops.len() {
        match ops[i].as_str() {
            "addWord" => {
                let method_params = split_array(&params[i])?;
                let word: String = deserialize(&method_params[0])?;
                obj.add_word(word);
                output.push("null".to_string());
            }
            "search" => {
                let method_params = split_array(&params[i])?;
                let word: String = deserialize(&method_params[0])?;
                let ans: bool = obj.search(word).into();
                output.push(serialize(ans)?);
            }
            _ => panic!("unknown op"),
        }
    }

    println!("\noutput: {}", join_array(output));
    Ok(())
}
