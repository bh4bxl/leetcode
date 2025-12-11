// Created by bh4bxl at 2025/12/10 13:38
// leetgo: 1.4.15
// https://leetcode.com/problems/coin-change/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let len = coins.len();
        let mut dp = vec![i32::MAX >> 1; amount as usize + 1];

        dp[0] = 0;

        for &coin in &coins {
            if coin <= amount {
                dp[coin as usize] = 1;
            }
        }

        for i in 1..=amount as usize {
            if dp[i] == 1 {
                continue;
            }

            for &coin in &coins {
                dp[i] = dp[i].min(if i as i32 - coin >= 0 {
                    dp[i - coin as usize] + 1
                } else {
                    i32::MAX >> 1
                })
            }
        }

        if dp[amount as usize] == i32::MAX >> 1 {
            -1
        } else {
            dp[amount as usize]
        }
    }
}

// @lc code=end

fn main() -> Result<()> {
    let coins: Vec<i32> = deserialize(&read_line()?)?;
    let amount: i32 = deserialize(&read_line()?)?;
    let ans: i32 = Solution::coin_change(coins, amount).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
