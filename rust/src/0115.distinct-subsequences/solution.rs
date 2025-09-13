// Created by bh4bxl at 2025/09/09 21:49
// leetgo: 1.4.15
// https://leetcode.com/problems/distinct-subsequences/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let (ls, lt) = (s.len(), t.len());
        let mut dp = vec![vec![0; lt + 1]; ls + 1];

        for i in 0..ls {
            dp[i][0] = 1;
        }

        let bs: Vec<u8> = s.as_bytes().to_vec();
        let bt: Vec<u8> = t.as_bytes().to_vec();

        for i in 1..=ls {
            for j in 1..=lt {
                dp[i][j] += dp[i - 1][j];
                if bs[i - 1] == bt[j - 1] {
                    dp[i][j] = dp[i - 1][j] + dp[i - 1][j - 1];
                }
            }
        }

        dp[ls][lt]
    }
}

// @lc code=end

fn main() -> Result<()> {
    let s: String = deserialize(&read_line()?)?;
    let t: String = deserialize(&read_line()?)?;
    let ans: i32 = Solution::num_distinct(s, t).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
