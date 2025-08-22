// Created by bh4bxl at 2025/07/29 16:26
// leetgo: 1.4.15
// https://leetcode.com/problems/spiral-matrix/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        enum DIR {
            RIGHT,
            DOWN,
            LEFT,
            UP,
        }
        let mut res = vec![];
        let (mtx_width, mtx_heigh) = (matrix[0].len(), matrix.len());
        let (mut top, mut right, mut bottom, mut left) = (0, mtx_width, mtx_heigh, 0);
        let (mut y, mut x) = (0, 0);
        let mut dir = DIR::RIGHT;

        while res.len() < mtx_width * mtx_heigh {
            res.push(matrix[y][x]);
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
                        dir = DIR::LEFT;
                    } else {
                        y += 1;
                    }
                }
                DIR::LEFT => {
                    if x == left {
                        y -= 1;
                        bottom -= 1;
                        dir = DIR::UP;
                    } else {
                        x -= 1;
                    }
                }
                DIR::UP => {
                    if y == top {
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
    let matrix: Vec<Vec<i32>> = deserialize(&read_line()?)?;
    let ans: Vec<i32> = Solution::spiral_order(matrix).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
