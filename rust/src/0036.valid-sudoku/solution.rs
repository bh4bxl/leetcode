// Created by bh4bxl at 2025/07/24 16:40
// leetgo: 1.4.13
// https://leetcode.com/problems/valid-sudoku/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        for i in 0..9 {
            let (mut chars_h, mut chars_v) = (vec![], vec![]);
            let mut chars_b = vec![];
            for j in 0..9 {
                let c_h = board[i][j];
                let c_v = board[j][i];
                let c_b = board[(i % 3) * 3 + j / 3][i - (i % 3) + (j % 3)];
                if c_h != '.' {
                    if chars_h.contains(&c_h) {
                        return false;
                    }
                    chars_h.push(c_h);
                }
                if c_v != '.' {
                    if chars_v.contains(&c_v) {
                        return false;
                    }
                    chars_v.push(c_v);
                }
                if c_b != '.' {
                    if chars_b.contains(&c_b) {
                        return false;
                    }
                    chars_b.push(c_b);
                }
            }
        }

        return true;
    }
}

// @lc code=end

fn main() -> Result<()> {
	let board: Vec<Vec<char>> = deserialize(&read_line()?)?;
	let ans: bool = Solution::is_valid_sudoku(board).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}
