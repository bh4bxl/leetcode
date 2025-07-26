// Created by bh4bxl at 2025/07/25 13:38
// leetgo: 1.4.15
// https://leetcode.com/problems/sudoku-solver/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        Self::solve(board);
    }

    fn solve(board: &mut Vec<Vec<char>>) -> bool {
        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] != '.' {
                    continue;
                }
                let mut c = '1';
                while c <= '9' {
                    if Self::valid(i, j, c, board) {
                        board[i][j] = c;
                        if Self::solve(board) {
                            return true;
                        } else {
                            board[i][j] = '.';
                        }
                    }
                    c = (c as u8 + 1) as char;
                }
                return false;
            }
        }
        true
    }

    fn valid(row: usize, col: usize, c: char, board: &Vec<Vec<char>>) -> bool {
        for i in 0..9 {
            if board[row][i] == c {
                return false;
            }
            if board[i][col] == c {
                return false;
            }
            if board[row / 3 * 3 + i / 3][col / 3 * 3 + i %3] == c {
                return false;
            }
        }
        true
    }
}

// @lc code=end

fn main() -> Result<()> {
	let board: Vec<Vec<char>> = deserialize(&read_line()?)?;
	Solution::solve_sudoku(board);
	let ans: Vec<Vec<char>> = board.into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}
