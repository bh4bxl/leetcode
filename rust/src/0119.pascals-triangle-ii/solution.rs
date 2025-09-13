// Created by bh4bxl at 2025/09/10 14:38
// leetgo: 1.4.15
// https://leetcode.com/problems/pascals-triangle-ii/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        if row_index == 0 {
            return vec![1];
        }
        let mut last = vec![1];

        for i in 1..=row_index as usize {
            let mut line = vec![0; i + 1];

            line[0] = 1;
            line[i] = 1;

            for j in 1..i {
                line[j] = last[j - 1] + last[j];
            }

            last = line;
        }

        last
    }
}

// @lc code=end

fn main() -> Result<()> {
    let row_index: i32 = deserialize(&read_line()?)?;
    let ans: Vec<i32> = Solution::get_row(row_index).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
