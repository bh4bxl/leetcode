// Created by bh4bxl at 2025/11/12 13:15
// leetgo: 1.4.15
// https://leetcode.com/problems/search-a-2d-matrix-ii/

use std::usize;

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let height = matrix.len() as i32;
        let (mut row, mut col) = (0, matrix[0].len() as i32 - 1);

        while row < height && col >= 0 {
            let val = matrix[row as usize][col as usize];
            if val == target {
                return true;
            } else if val > target {
                col -= 1;
            } else {
                row += 1;
            }
        }

        false
    }
}

// @lc code=end

// Warning: this is a manual question, the generated test code may be incorrect.
fn main() -> Result<()> {
    let matrix: Vec<Vec<i32>> = deserialize(&read_line()?)?;
    let target: i32 = deserialize(&read_line()?)?;
    let ans: bool = Solution::search_matrix(matrix, target).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
