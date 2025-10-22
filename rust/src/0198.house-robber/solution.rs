// Created by bh4bxl at 2025/10/20 11:32
// leetgo: 1.4.15
// https://leetcode.com/problems/house-robber/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut dp = vec![0; len + 1];

        for i in 1..=len {
            let temp = nums[i - 1] + (if i > 1 { dp[i - 2] } else { 0 });
            dp[i] = temp.max(dp[i - 1]);
        }

        dp[len]
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::rob(nums).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
