// Created by bh4bxl at 2025/09/12 13:14
// leetgo: 1.4.15
// https://leetcode.com/problems/surrounded-regions/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        if board.is_empty() || board[0].is_empty() {
            return;
        }

        let (rows, cols) = (board.len(), board[0].len());

        fn dfs(board: &mut Vec<Vec<char>>, r: usize, c: usize, rows: usize, cols: usize) {
            if r >= rows || c >= cols || board[r][c] != 'O' {
                return;
            }
            board[r][c] = '#';
            if r > 0 {
                dfs(board, r - 1, c, rows, cols);
            }
            if c > 0 {
                dfs(board, r, c - 1, rows, cols);
            }
            if r + 1 < rows {
                dfs(board, r + 1, c, rows, cols);
            }
            if c + 1 < cols {
                dfs(board, r, c + 1, rows, cols);
            }
        }

        for r in 0..rows {
            dfs(board, r, 0, rows, cols);
            dfs(board, r, cols - 1, rows, cols);
        }
        for c in 0..cols {
            dfs(board, 0, c, rows, cols);
            dfs(board, rows - 1, c, rows, cols);
        }

        for r in 0..rows {
            for c in 0..cols {
                if board[r][c] == 'O' {
                    board[r][c] = 'X';
                } else if board[r][c] == '#' {
                    board[r][c] = 'O';
                }
            }
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let mut board: Vec<Vec<char>> = deserialize(&read_line()?)?;
    Solution::solve(&mut board);
    let ans: Vec<Vec<char>> = board.into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
