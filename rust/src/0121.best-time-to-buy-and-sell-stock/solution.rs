// Created by bh4bxl at 2025/09/10 16:37
// leetgo: 1.4.15
// https://leetcode.com/problems/best-time-to-buy-and-sell-stock/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max_profit = 0;
        let mut min_price = i32::MAX;
        let len = prices.len();

        for i in 0..len {
            min_price = std::cmp::min(min_price, prices[i]);
            max_profit = std::cmp::max(max_profit, prices[i] - min_price);
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
