// Created by bh4bxl at 2025/08/01 16:02
// leetgo: 1.4.15
// https://leetcode.com/problems/unique-paths/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut dp = vec![vec![0; n as usize]; m as usize];

        for i in 0..m as usize {
            dp[i][0] = 1;
        }
        for i in 0..n as usize {
            dp[0][i] = 1;
        }
        for i in 1..m as usize {
            for j in 1..n as usize {
                dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
            }
        }
        dp[m as usize - 1][n as usize - 1]
    }
}

// @lc code=end

fn main() -> Result<()> {
    let m: i32 = deserialize(&read_line()?)?;
    let n: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::unique_paths(m, n).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
