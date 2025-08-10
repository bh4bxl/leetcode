// Created by bh4bxl at 2025/08/10 12:40
// leetgo: 1.4.15
// https://leetcode.com/problems/word-search/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        let chars: Vec<char> = word.chars().collect();
        let mut visited = vec![vec![false; board[0].len()]; board.len()];
        let idx = 0;

        for i in 0..board.len() {
            for j in 0..board[0].len() {
                if Self::is_word((i, j), &board, idx, &chars, &mut visited) {
                    return true;
                }
            }
        }

        false
    }

    fn is_word(
        pos: (usize, usize),
        board: &Vec<Vec<char>>,
        idx: usize,
        chars: &Vec<char>,
        visited: &mut Vec<Vec<bool>>,
    ) -> bool {
        if visited[pos.0][pos.1] || board[pos.0][pos.1] != chars[idx] {
            return false;
        }

        if idx == chars.len() - 1 {
            return true;
        }

        visited[pos.0][pos.1] = true;
        let mut nexts = vec![];
        if pos.0 != 0 {
            nexts.push((pos.0 - 1, pos.1));
        }
        if pos.0 != board.len() - 1 {
            nexts.push((pos.0 + 1, pos.1));
        }
        if pos.1 != 0 {
            nexts.push((pos.0, pos.1 - 1));
        }
        if pos.1 != board[0].len() - 1 {
            nexts.push((pos.0, pos.1 + 1));
        }
        for next in nexts {
            if Self::is_word(next, board, idx + 1, chars, visited) {
                return true;
            }
        }
        visited[pos.0][pos.1] = false;
        false
    }
}

// @lc code=end

fn main() -> Result<()> {
    let board: Vec<Vec<char>> = deserialize(&read_line()?)?;
    let word: String = deserialize(&read_line()?)?;
    let ans: bool = Solution::exist(board, word).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
