// Created by bh4bxl at 2025/10/15 14:07
// leetgo: 1.4.15
// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iv/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        let len = prices.len();
        if len < 2 {
            return 0;
        }
        if k > len as i32 {
            let mut buys = vec![0; len + 1];
            let mut sells = vec![0; len + 1];
            buys[1] = -prices[0];
            for i in 2..=len {
                buys[i] = buys[i - 1].max(sells[i - 1] - prices[i - 1]);
                sells[i] = sells[i - 1].max(buys[i - 1] + prices[i - 1]);
            }

            return sells[len];
        }

        let mut buys = vec![vec![0; k as usize + 1]; len + 1];
        let mut sells = vec![vec![0; k as usize + 1]; len + 1];
        for i in 2..=k as usize {
            for j in 0..i {
                buys[j][i] = i32::MIN;
            }
        }
        let mut sum = 0;
        for i in 1..=k as usize {
            sum -= prices[i - 1];
            buys[i][i] = sum;
        }
        for i in 2..=len {
            for j in 1..=k as usize {
                buys[i][j] = buys[i - 1][j].max(sells[i - 1][j - 1] - prices[i - 1]);
                sells[i][j] = sells[i - 1][j].max(buys[i - 1][j] + prices[i - 1]);
            }
        }
        let mut profit = 0;
        for i in 0..=k as usize {
            profit = profit.max(sells[len][i]);
        }

        profit
    }
}

// @lc code=end

fn main() -> Result<()> {
    let k: i32 = deserialize(&read_line()?)?;
    let prices: Vec<i32> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::max_profit(k, prices).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
