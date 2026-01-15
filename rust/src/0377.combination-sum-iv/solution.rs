// Created by bh4bxl at 2026/01/14 17:22
// leetgo: 1.4.16
// https://leetcode.com/problems/combination-sum-iv/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let target = target as usize;
        let mut dp = vec![0i64; target + 1];
        dp[0] = 1;

        for i in 1..=target {
            for &num in &nums {
                let num = num as usize;
                if i >= num {
                    dp[i] += dp[i - num];
                }
            }
        }

        dp[target] as i32
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let target: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::combination_sum4(nums, target).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
