// Created by bh4bxl at 2026/01/13 17:50
// leetgo: 1.4.16
// https://leetcode.com/problems/guess-number-higher-or-lower-ii/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn get_money_amount(n: i32) -> i32 {
        let n = n as usize;
        let mut dp = vec![vec![0; n + 2]; n + 2];

        // length = interval size
        for len in 2..=n {
            for l in 1..=n - len + 1 {
                let r = l + len - 1;
                dp[l][r] = i32::MAX;

                for x in l..=r {
                    let cost = x as i32 + dp[l][x - 1].max(dp[x + 1][r]);
                    dp[l][r] = dp[l][r].min(cost);
                }
            }
        }

        dp[1][n]
    }
}

// @lc code=end

fn main() -> Result<()> {
    let n: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::get_money_amount(n).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
