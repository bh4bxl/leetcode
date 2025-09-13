// Created by bh4bxl at 2025/09/10 20:18
// leetgo: 1.4.15
// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;

        for i in 1..prices.len() {
            if prices[i] > prices[i - 1] {
                max_profit += prices[i] - prices[i - 1];
            }
        }

        max_profit
    }
}

// @lc code=end

fn main() -> Result<()> {
    let prices: Vec<i32> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::max_profit(prices).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
