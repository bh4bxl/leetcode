// Created by bh4bxl at 2025/07/28 18:12
// leetgo: 1.4.15
// https://leetcode.com/problems/rotate-image/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let n = matrix.len();

        for i in 0..n / 2 {
            matrix.swap(i, n - 1 - i);
        }

        for i in 0..n {
            for j in 0..i + 1 {
                if i == j {
                    continue;
                }
                let bak = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = bak;
            }
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
	let matrix: Vec<Vec<i32>> = deserialize(&read_line()?)?;
	Solution::rotate(matrix);
	let ans: Vec<Vec<i32>> = matrix.into();

	println!("\noutput: {}", serialize(ans)?);
	Ok(())
}
