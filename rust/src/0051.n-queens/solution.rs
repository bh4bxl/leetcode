// Created by bh4bxl at 2025/07/29 12:24
// leetgo: 1.4.15
// https://leetcode.com/problems/n-queens/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut res: Vec<Vec<String>> = vec![];
        let size = n as usize;
        let mut board = vec![".".repeat(size); size];

        Self::backtrace(&mut res, &mut board, 0, size);

        res
    }

    fn backtrace(res: &mut Vec<Vec<String>>, board: &mut Vec<String>, i: usize, size: usize) {
        if i == size {
            res.push(board.clone());
            return;
        }

        for j in 0..size {
            if Self::is_valid(board, size, i, j) {
                board[i] = Self::replace_nth_char(&board[i], j, 'Q');
                Self::backtrace(res, board, i + 1, size);
                board[i] = Self::replace_nth_char(&board[i], j, '.');
            }
        }
    }

    fn replace_nth_char(s: &str, n: usize, new_c: char) -> String {
        s.chars()
            .enumerate()
            .map(|(i, c)| {
                if i == n {
                    new_c
                } else {
                    c
                }
            })
            .collect()
    }

    fn is_valid(board: &Vec<String>, size: usize, row: usize, col: usize) -> bool {
        if board[row].contains('Q') {
            return false;
        }
        for i in 0..size {
            if board[i].chars().nth(col).unwrap() == 'Q' {
                return false;
            }
        }

        let (mut x, mut y) = (row, col);
        while x < size && y < size {
            if board[x].chars().nth(y).unwrap() == 'Q' {
                return false;
            }
            x -= 1;
            y -= 1;
        }
        (x, y) = (row, col);
        while x < size && y < size {
            if board[x].chars().nth(y).unwrap() == 'Q' {
                return false;
            }
            x -= 1;
            y += 1;
        }

        true
    }
}

// @lc code=end

fn main() -> Result<()> {
	let n: i32 = deserialize(&read_line()?)?;
	let ans: Vec<Vec<String>> = Solution::solve_nqueens(n).into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}
