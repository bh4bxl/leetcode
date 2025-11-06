// Created by bh4bxl at 2025/11/03 10:04
// leetgo: 1.4.15
// https://leetcode.com/problems/maximal-square/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let (height, width) = (matrix.len(), matrix[0].len());
        let mut max = 0;
        let mut dp = vec![vec![0; width]; height];

        for i in 0..height {
            for j in 0..width {
                if matrix[i][j] == '1' {
                    if i > 0 && j > 0 {
                        if dp[i - 1][j] == 0 || dp[i][j - 1] == 0 || dp[i - 1][j - 1] == 0 {
                            dp[i][j] = 1;
                        } else {
                            dp[i][j] = dp[i - 1][j].min(dp[i][j - 1].min(dp[i - 1][j - 1])) + 1;
                        }
                    } else {
                        dp[i][j] = 1;
                    }
                }
                max = max.max(dp[i][j]);
            }
        }

        max * max
    }
}

// @lc code=end

fn main() -> Result<()> {
    let matrix: Vec<Vec<char>> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::maximal_square(matrix).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
