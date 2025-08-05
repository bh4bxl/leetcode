// Created by bh4bxl at 2025/08/04 14:26
// leetgo: 1.4.15
// https://leetcode.com/problems/minimum-path-sum/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut dp = vec![vec![0; n]; m];

        dp[0][0] = grid[0][0];
        for i in 1..m {
            dp[i][0] = dp[i - 1][0] + grid[i][0];
        }

        for i in 1..n {
            dp[0][i] = dp[0][i - 1] + grid[0][i];
        }

        for i in 1..m {
            for j in 1..n {
                dp[i][j] = std::cmp::min(dp[i][j - 1], dp[i - 1][j]) + grid[i][j];
            }
        }

        dp[m - 1][n - 1]
    }
}

// @lc code=end

fn main() -> Result<()> {
    let grid: Vec<Vec<i32>> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::min_path_sum(grid).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
