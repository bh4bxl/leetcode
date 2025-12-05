// Created by bh4bxl at 2025/12/02 19:06
// leetgo: 1.4.15
// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-cooldown/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let len = prices.len();
        let mut buy = vec![0; len + 1];
        let mut sell = vec![0; len + 1];
        buy[1] = -prices[0];
        for i in 2..=len {
            buy[i] = buy[i - 1].max(sell[i - 2] - prices[i - 1]);
            sell[i] = sell[i - 1].max(buy[i - 1] + prices[i - 1]);
        }

        *sell.last().unwrap()
    }
}

// @lc code=end

fn main() -> Result<()> {
    let prices: Vec<i32> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::max_profit(prices).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
