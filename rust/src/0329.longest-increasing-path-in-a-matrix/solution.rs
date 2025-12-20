// Created by bh4bxl at 2025/12/16 23:15
// leetgo: 1.4.15
// https://leetcode.com/problems/longest-increasing-path-in-a-matrix/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut dp = vec![vec![0; n]; m];

        fn dfs(i: usize, j: usize, matrix: &Vec<Vec<i32>>, dp: &mut Vec<Vec<i32>>) -> i32 {
            if dp[i][j] != 0 {
                return dp[i][j];
            }

            let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
            let mut best = 1;

            for (dx, dy) in dirs {
                let (ni, nj) = (i as i32 + dx, j as i32 + dy);

                if ni >= 0
                    && ni < matrix.len() as i32
                    && nj >= 0
                    && nj < matrix[0].len() as i32
                    && matrix[ni as usize][nj as usize] > matrix[i][j]
                {
                    best = best.max(1 + dfs(ni as usize, nj as usize, matrix, dp));
                }
            }

            dp[i][j] = best;
            best
        }

        let mut res = 0;
        for i in 0..m {
            for j in 0..n {
                res = res.max(dfs(i, j, &matrix, &mut dp));
            }
        }

        res
    }
}

// @lc code=end

fn main() -> Result<()> {
    let matrix: Vec<Vec<i32>> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::longest_increasing_path(matrix).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
