// Created by bh4bxl at 2025/10/27 14:28
// leetgo: 1.4.15
// https://leetcode.com/problems/house-robber-ii/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let len = nums.len();

        if len == 1 {
            return nums[0];
        }

        let mut res = 0;
        let mut dp = vec![0; len + 1];

        for i in 1..len {
            dp[i] = dp[i - 1].max(nums[i - 1] + (if i >= 2 { dp[i - 2] } else { 0 }));
        }

        let max = dp[len - 1];
        dp = vec![0; len + 1];
        for i in 2..=len {
            dp[i] = dp[i - 1].max(nums[i - 1] + dp[i - 2]);
        }

        max.max(dp[len])
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::rob(nums).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
