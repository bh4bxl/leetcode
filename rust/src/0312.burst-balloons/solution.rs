// Created by bh4bxl at 2025/12/09 13:28
// leetgo: 1.4.15
// https://leetcode.com/problems/burst-balloons/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn max_coins(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut dp = vec![vec![0; len + 2]; len + 2];
        let mut arr = vec![1; len + 2];

        for i in 1..=len {
            arr[i] = nums[i - 1];
            dp[i][i] = nums[i - 1];
        }

        for i in 1..=len {
            for left in 1..=len - i + 1 {
                let right = left + i - 1;
                for mid in left..=right {
                    dp[left][right] = dp[left][right].max(
                        dp[left][mid - 1]
                            + arr[left - 1] * arr[mid] * arr[right + 1]
                            + dp[mid + 1][right],
                    );
                }
            }
        }

        dp[1][len]
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::max_coins(nums).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
