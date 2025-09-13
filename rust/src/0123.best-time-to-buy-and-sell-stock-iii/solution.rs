// Created by bh4bxl at 2025/09/10 20:27
// leetgo: 1.4.15
// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iii/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut local = vec![0; 3];
        let mut global = vec![0; 3];

        for i in 0..prices.len() - 1 {
            let diff = prices[i + 1] - prices[i];
            for j in (1..=2).rev() {
                local[j] = std::cmp::max(global[j - 1] + std::cmp::max(diff, 0), local[j] + diff);
                global[j] = std::cmp::max(global[j], local[j]);
            }
        }

        global[2]
    }
}

// @lc code=end

fn main() -> Result<()> {
    let prices: Vec<i32> = deserialize(&read_line()?)?;
    let ans: i32 = Solution::max_profit(prices).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
