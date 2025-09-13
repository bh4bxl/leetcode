// Created by bh4bxl at 2025/09/10 15:37
// leetgo: 1.4.15
// https://leetcode.com/problems/triangle/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let len = triangle.len();
        let mut dp = vec![vec![i32::MAX; len]; len];

        dp[0][0] = triangle[0][0];

        for i in 1..len {
            dp[i][0] = dp[i - 1][0] + triangle[i][0];
            dp[i][i] = dp[i - 1][i - 1] + triangle[i][i];
        }

        for i in 2..len {
            for j in 1..i {
                dp[i][j] = std::cmp::min(dp[i - 1][j - 1], dp[i - 1][j]) + triangle[i][j];
            }
        }

        dp[len - 1].sort();

        dp[len - 1][0]
    }
}

// @lc code=end

fn main() -> Result<()> {
    let triangle: Vec<Vec<i32>> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::minimum_total(triangle).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
