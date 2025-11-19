// Created by bh4bxl at 2025/11/18 14:42
// leetgo: 1.4.15
// https://leetcode.com/problems/missing-number/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut sum = len * (len + 1) / 2;

        for n in nums {
            sum -= n as usize;
        }

        sum as i32
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::missing_number(nums).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
