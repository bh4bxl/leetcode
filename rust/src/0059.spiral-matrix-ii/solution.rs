// Created by bh4bxl at 2025/07/31 15:39
// leetgo: 1.4.15
// https://leetcode.com/problems/spiral-matrix-ii/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let size = n as usize;
        let mut res = vec![vec![0; size]; size];
        let (mut top, mut right, mut bottom, mut left) = (0, size, size, 0);
        let (mut x, mut y) = (0, 0);
        enum DIR {
            RIGHT,
            DOWN,
            LEFT,
            UP,
        }
        let mut dir = DIR::RIGHT;

        for i in 1..n * n + 1 {
            res[y][x] = i;
            match dir {
                DIR::RIGHT => {
                    if x + 1 == right {
                        y += 1;
                        top += 1;
                        dir = DIR::DOWN;
                    } else {
                        x += 1;
                    }
                }
                DIR::DOWN => {
                    if y + 1 == bottom {
                        x -= 1;
                        right -= 1;
                        dir = DIR::LEFT
                    } else {
                        y += 1;
                    }
                }
                DIR::LEFT => {
                    if x - 1 == usize::MAX || x - 1 < left {
                        y -= 1;
                        bottom -= 1;
                        dir = DIR::UP;
                    } else {
                        x -= 1;
                    }
                }
                DIR::UP => {
                    if y - 1 < top {
                        x += 1;
                        left += 1;
                        dir = DIR::RIGHT;
                    } else {
                        y -= 1;
                    }
                }
            }
        }

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
    let n: i32 = deserialize(&read_line()?)?;
    let ans: Vec<Vec<i32>> = Solution::generate_matrix(n).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
