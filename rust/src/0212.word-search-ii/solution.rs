// Created by bh4bxl at 2025/10/27 13:39
// leetgo: 1.4.15
// https://leetcode.com/problems/word-search-ii/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

#[derive(Default)]
struct TrieNode {
    children: std::collections::HashMap<char, TrieNode>,
    word: Option<String>,
}

impl TrieNode {
    fn insert(&mut self, word: &str) {
        let mut node = self;
        for c in word.chars() {
            node = node.children.entry(c).or_default();
        }
        node.word = Some(word.to_string());
    }
}

impl Solution {
    pub fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        let mut root = TrieNode::default();
        for word in &words {
            root.insert(word);
        }

        let mut res = std::collections::HashSet::new();
        let mut board = board;
        let rows = board.len();
        let cols = board[0].len();

        for i in 0..rows {
            for j in 0..cols {
                Self::dfs(i as i32, j as i32, &mut board, &mut root, &mut res);
            }
        }

        res.into_iter().collect()
    }

    fn dfs(
        i: i32,
        j: i32,
        board: &mut Vec<Vec<char>>,
        node: &mut TrieNode,
        res: &mut std::collections::HashSet<String>,
    ) {
        if i < 0
            || j < 0
            || i as usize >= board.len()
            || j as usize >= board[0].len()
            || board[i as usize][j as usize] == '#'
        {
            return;
        }

        let c = board[i as usize][j as usize];
        if !node.children.contains_key(&c) {
            return;
        }

        let next_node = node.children.get_mut(&c).unwrap();
        if let Some(word) = &next_node.word {
            res.insert(word.clone());
        }

        board[i as usize][j as usize] = '#';

        let dir = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for (dx, dy) in dir {
            Self::dfs(i + dx, j + dy, board, next_node, res);
        }

        board[i as usize][j as usize] = c;
    }
}

// @lc code=end

fn main() -> Result<()> {
    let board: Vec<Vec<char>> = deserialize(&read_line()?)?;
    let words: Vec<String> = deserialize(&read_line()?)?;
    let ans: Vec<String> = Solution::find_words(board, words).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
