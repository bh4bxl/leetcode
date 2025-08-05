// Created by bh4bxl at 2025/08/04 13:50
// leetgo: 1.4.15
// https://leetcode.com/problems/unique-paths-ii/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (obstacle_grid.len(), obstacle_grid[0].len());
        let mut dp = vec![vec![0; n]; m];

        for i in 0..m {
            if obstacle_grid[i][0] != 1 {
                dp[i][0] = 1;
            } else {
                break;
            }
        }

        for i in 0..n {
            if obstacle_grid[0][i] != 1 {
                dp[0][i] = 1;
            } else {
                break;
            }
        }

        for i in 1..m {
            for j in 1..n {
                if obstacle_grid[i][j] != 1 {
                    dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
                }
            }
        }

        dp[m - 1][n - 1]
    }
}

// @lc code=end

fn main() -> Result<()> {
    let obstacle_grid: Vec<Vec<i32>> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::unique_paths_with_obstacles(obstacle_grid).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
