// Created by bh4bxl at 2025/08/14 20:48
// leetgo: 1.4.15
// https://leetcode.com/problems/gray-code/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let mut gray = vec![];

        for i in 0..(1 << n) {
            gray.push(i ^ i >> 1);
        }

        gray
    }
}

// @lc code=end

fn main() -> Result<()> {
    let n: i32 = deserialize(&read_line()?)?;
    let ans: Vec<i32> = Solution::gray_code(n).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
