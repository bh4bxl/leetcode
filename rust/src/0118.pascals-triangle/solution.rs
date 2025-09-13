// Created by bh4bxl at 2025/09/10 13:05
// leetgo: 1.4.15
// https://leetcode.com/problems/pascals-triangle/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];

        res.push(vec![1]);

        for i in 1..num_rows as usize {
            let mut line = vec![0; i + 1];

            line[0] = 1;
            line[i] = 1;

            for j in 1..i {
                line[j] = res[i - 1][j - 1] + res[i - 1][j];
            }

            res.push(line);
        }

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
    let num_rows: i32 = deserialize(&read_line()?)?;
    let ans: Vec<Vec<i32>> = Solution::generate(num_rows).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
