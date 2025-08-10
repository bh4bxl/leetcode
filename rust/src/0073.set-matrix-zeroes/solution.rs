// Created by bh4bxl at 2025/08/08 10:25
// leetgo: 1.4.15
// https://leetcode.com/problems/set-matrix-zeroes/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let (mut cols, mut rows) = (vec![], vec![]);
        for (i, col) in matrix.iter().enumerate() {
            for (j, item) in col.iter().enumerate() {
                if *item == 0 {
                    cols.push(i);
                    rows.push(j);
                }
            }
        }
        for i in 0..matrix.len() {
            for j in 0..matrix[0].len() {
                if cols.contains(&i) || rows.contains(&j) {
                    matrix[i][j] = 0;
                }
            }
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let mut matrix: Vec<Vec<i32>> = deserialize(&read_line()?)?;
    Solution::set_zeroes(&mut matrix);
    let ans: Vec<Vec<i32>> = matrix.into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
