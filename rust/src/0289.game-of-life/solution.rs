// Created by bh4bxl at 2025/11/27 12:09
// leetgo: 1.4.15
// https://leetcode.com/problems/game-of-life/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        #[rustfmt::skip]
        let dirs = [
            (-1, -1), (-1,  0),  (-1,  1),
            ( 0, -1),            ( 0,  1),
            ( 1, -1), ( 1,  0),  ( 1,  1)
        ];

        let (width, heigh) = (board[0].len(), board.len());

        for i in 0..heigh {
            for j in 0..width {
                let mut live_neighbors = 0;

                for &(dx, dy) in &dirs {
                    let (x, y) = (i as i32 + dx, j as i32 + dy);

                    if x >= 0 && x < heigh as i32 && y >= 0 && y < width as i32 {
                        let v = board[x as usize][y as usize];
                        if v == 1 || v == -1 {
                            live_neighbors += 1;
                        }
                    }
                }

                let cell = board[i][j];

                if cell == 1 && (live_neighbors < 2 || live_neighbors > 3) {
                    board[i][j] = -1;
                }
                if cell == 0 && live_neighbors == 3 {
                    board[i][j] = 2;
                }
            }
        }

        for i in 0..heigh {
            for j in 0..width {
                board[i][j] = if board[i][j] > 0 { 1 } else { 0 };
            }
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let mut board: Vec<Vec<i32>> = deserialize(&read_line()?)?;
    Solution::game_of_life(&mut board);
    let ans: Vec<Vec<i32>> = board.into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
